#![expect(dead_code)]

use std::{
    io,
    os::fd::OwnedFd,
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

use anyhow::Context;
use ashpd::desktop::{
    PersistMode, Session,
    remote_desktop::{DeviceType, KeyState, RemoteDesktop, SelectedDevices},
    screencast::{CursorMode, Screencast, SourceType, Stream},
};

use pipewire as pw;
use pw::{properties::properties, spa};
use spa::param::video::VideoInfoRaw;

use tokio::fs;
use tokio::io::AsyncWriteExt;

use super::key::{Key, press_key, press_key_with_modifiers};

pub struct Robot {
    session: Session<'static, RemoteDesktop>,
    remote: RemoteDesktop,
    screencast: Screencast,
    response: SelectedDevices,
    stream: Stream,
    streaming_thread: Option<JoinHandle<()>>,
}

pub struct Frame {
    pub format: VideoInfoRaw,
    pub buffer: Vec<u8>,
    pub stride: u32,
}

fn streaming_thread(
    fd: OwnedFd,
    stream: Stream,
    tx: mpsc::SyncSender<Frame>,
) -> anyhow::Result<()> {
    let node_id = stream.pipe_wire_node_id();

    pw::init();
    let mainloop = pw::main_loop::MainLoopBox::new(None).context("Failed to create main loop")?;
    let context =
        pw::context::ContextBox::new(mainloop.loop_(), None).context("Failed to create context")?;
    let core = context
        .connect_fd(fd, None)
        .context("Failed to connect file descriptor")?;

    let stream = pw::stream::StreamBox::new(
        &core,
        "wlrobot",
        properties! {
            *pw::keys::MEDIA_TYPE => "Video",
            *pw::keys::MEDIA_CATEGORY => "Capture",
            *pw::keys::MEDIA_ROLE => "Screen",
        },
    )
    .context("Failed to create stream")?;

    let format_data: spa::param::video::VideoInfoRaw = Default::default();
    let _listener = stream
        .add_local_listener_with_user_data(format_data)
        .state_changed(|_, _, old, new| {
            println!("State changed: {:?} -> {:?}", old, new);
        })
        .param_changed(|_, format_data, id, param| {
            let Some(param) = param else {
                return;
            };
            if id != pw::spa::param::ParamType::Format.as_raw() {
                return;
            }

            let (media_type, media_subtype) =
                match pw::spa::param::format_utils::parse_format(param) {
                    Ok(v) => v,
                    Err(_) => return,
                };

            if media_type != pw::spa::param::format::MediaType::Video
                || media_subtype != pw::spa::param::format::MediaSubtype::Raw
            {
                return;
            }

            format_data.parse(param).expect("Failed to parse param");

            println!("got video format:");
            println!(
                "\tformat: {} ({:?})",
                format_data.format().as_raw(),
                format_data.format()
            );
            println!(
                "\tsize: {}x{}",
                format_data.size().width,
                format_data.size().height
            );
            println!(
                "\tframerate: {}/{}",
                format_data.framerate().num,
                format_data.framerate().denom
            );

            // prepare to render video of this size
        })
        .process(move |stream, format_data| match stream.dequeue_buffer() {
            None => println!("out of buffers"),
            Some(mut buffer) => {
                let datas = buffer.datas_mut();
                let data = &mut datas[0];
                let Some(slice) = data.data() else {
                    return;
                };

                tx.send(Frame {
                    format: *format_data,
                    buffer: Vec::from(slice),
                    stride: data.chunk().stride() as u32,
                })
                .expect("TODO: implement proper stream teardown");
            }
        })
        .register()
        .context("Failed to register stream listener")?;

    println!("Created stream {:#?}", stream);

    let obj = pw::spa::pod::object!(
        pw::spa::utils::SpaTypes::ObjectParamFormat,
        pw::spa::param::ParamType::EnumFormat,
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::MediaType,
            Id,
            pw::spa::param::format::MediaType::Video
        ),
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::MediaSubtype,
            Id,
            pw::spa::param::format::MediaSubtype::Raw
        ),
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::VideoFormat,
            Choice,
            Enum,
            Id,
            pw::spa::param::video::VideoFormat::RGB,
            pw::spa::param::video::VideoFormat::RGB,
            pw::spa::param::video::VideoFormat::RGBA,
            pw::spa::param::video::VideoFormat::RGBx,
            pw::spa::param::video::VideoFormat::BGRx,
            pw::spa::param::video::VideoFormat::YUY2,
            pw::spa::param::video::VideoFormat::I420,
        ),
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::VideoSize,
            Choice,
            Range,
            Rectangle,
            pw::spa::utils::Rectangle {
                width: 320,
                height: 240
            },
            pw::spa::utils::Rectangle {
                width: 1,
                height: 1
            },
            pw::spa::utils::Rectangle {
                width: 4096,
                height: 4096
            }
        ),
        pw::spa::pod::property!(
            pw::spa::param::format::FormatProperties::VideoFramerate,
            Choice,
            Range,
            Fraction,
            pw::spa::utils::Fraction { num: 25, denom: 1 },
            pw::spa::utils::Fraction { num: 0, denom: 1 },
            pw::spa::utils::Fraction {
                num: 1000,
                denom: 1
            }
        ),
    );
    let values: Vec<u8> = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(obj),
    )
    .unwrap()
    .0
    .into_inner();

    let mut params = [spa::pod::Pod::from_bytes(&values).unwrap()];

    stream
        .connect(
            spa::utils::Direction::Input,
            Some(node_id),
            pw::stream::StreamFlags::AUTOCONNECT | pw::stream::StreamFlags::MAP_BUFFERS,
            &mut params,
        )
        .context("Failed to connect stream")?;

    println!("Connected stream");

    mainloop.run();
    Ok(())
}

impl Robot {
    pub async fn new() -> anyhow::Result<Self> {
        let state_dir = dirs::state_dir().unwrap().join("wlrobot");
        fs::create_dir_all(&state_dir)
            .await
            .with_context(|| format!("Failed to create state directory {:?}", state_dir))?;
        let tok_file = state_dir.join("token");

        let restore_token = match fs::read_to_string(&tok_file).await {
            Ok(tok) => Some(tok),
            Err(err) if err.kind() == io::ErrorKind::NotFound => None,
            Err(err) => return Err(err.into()),
        };

        let rd_proxy_1 = RemoteDesktop::new()
            .await
            .context("Failed to create RemoteDesktop 1")?;
        let rd_session_1 = rd_proxy_1
            .create_session()
            .await
            .context("Failed to create remote desktop session 1")?;
        rd_proxy_1
            .select_devices(
                &rd_session_1,
                DeviceType::Keyboard | DeviceType::Pointer,
                restore_token.as_deref(),
                PersistMode::ExplicitlyRevoked,
            )
            .await
            .context("Failed to select input devices to remote control session 1")?;

        let response_1 = rd_proxy_1
            .start(&rd_session_1, None)
            .await
            .context("Failed to start remote desktop session 1")?
            .response()
            .context("Failed to get remote desktop session response 1")?;

        if restore_token.as_deref() != response_1.restore_token()
            && let Some(tok) = response_1.restore_token()
        {
            let mut f = fs::File::create(&tok_file)
                .await
                .with_context(|| format!("Failed to create restore token file {:?}", tok_file))?;
            f.write(tok.as_bytes())
                .await
                .with_context(|| format!("Failed to write {:?} to tok file {:?}", tok, tok_file))?;
        }

        let rd_proxy = RemoteDesktop::new()
            .await
            .context("Failed to create RemoteDesktop")?;
        let session = rd_proxy
            .create_session()
            .await
            .context("Failed to create remote desktop session")?;
        rd_proxy
            .select_devices(
                &session,
                DeviceType::Keyboard | DeviceType::Pointer,
                None,
                PersistMode::DoNot,
            )
            .await
            .context("Failed to select input devices to remote control session")?;

        let sc_proxy = Screencast::new()
            .await
            .context("Failed to create Screencast")?;
        sc_proxy
            .select_sources(
                &session,
                CursorMode::Embedded,
                SourceType::Monitor.into(),
                false, // multiple
                None,  // restore_token
                PersistMode::DoNot,
            )
            .await
            .context("Failed to configure screen cast session")?;

        // This is a hack to work around the fact that you're not supposed to
        // be able to persist the screencast permission...
        // Nobody tell GNOME, this should probably be considered an exploit
        tokio::spawn({
            let proxy = rd_proxy_1;
            let session = rd_session_1;

            // Try to press a key. If an error occurred, print it and return. Sleeps after key is pressed
            macro_rules! try_press_sleep {
                ($key:expr) => {
                    async {
                        eprintln!("Pressing {}...", $key);
                        if let Err(err) = press_key(&proxy, &session, $key).await {
                            eprintln!("{}", err);
                            return;
                        }
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                };
                ($($modifier:expr),+; $key:expr) => {
                    async {
                        eprintln!("Pressing {:?} + {}...", &[$($modifier),+], $key);
                        if let Err(err) = press_key_with_modifiers(&proxy, &session, $key, &[$($modifier),+]).await {
                            eprintln!("{}", err);
                            return;
                        }
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                };
            }

            async move {
                // Space to activate remote desktop
                try_press_sleep!(Key::Space).await;

                // Tab + space to select (first) screen
                try_press_sleep!(Key::Tab).await;
                try_press_sleep!(Key::Space).await;

                // Shift-Tab twice to focus the share button
                try_press_sleep!(Key::LeftShift; Key::Tab).await;
                try_press_sleep!(Key::LeftShift; Key::Tab).await;

                // Space to allow sharing
                try_press_sleep!(Key::Space).await;
            }
        });

        let response = rd_proxy
            .start(&session, None)
            .await
            .context("Failed to start remote desktop session")?
            .response()
            .context("Failed to get remote desktop session response")?;

        let stream = match response.streams() {
            Some(streams) => streams.first(),
            None => None,
        };
        let Some(stream) = stream else {
            return Err(anyhow::anyhow!("Missing stream"));
        };
        let stream = stream.clone();

        Ok(Self {
            session,
            remote: rd_proxy,
            screencast: sc_proxy,
            response,
            stream,
            streaming_thread: None,
        })
    }

    pub async fn start_streaming(&mut self) -> anyhow::Result<mpsc::Receiver<Frame>> {
        if self.streaming_thread.is_some() {
            return Err(anyhow::anyhow!("Already streaming"));
        }

        let fd = self
            .screencast
            .open_pipe_wire_remote(&self.session)
            .await
            .context("Failed to open pipewire remote")?;
        let (tx, rx) = mpsc::sync_channel::<Frame>(0);

        self.streaming_thread = Some(thread::spawn({
            let stream = self.stream.clone();
            move || {
                if let Err(err) = streaming_thread(fd, stream, tx) {
                    println!("Streaming failed: {}", err);
                }
            }
        }));

        Ok(rx)
    }

    pub async fn press_key(&self, sym: i32) -> anyhow::Result<()> {
        self.remote
            .notify_keyboard_keysym(&self.session, sym, KeyState::Pressed)
            .await
            .with_context(|| format!("Failed to press key {sym}"))?;

        tokio::time::sleep(Duration::from_millis(10)).await;

        self.remote
            .notify_keyboard_keysym(&self.session, sym, KeyState::Released)
            .await
            .with_context(|| format!("Failed to release key {sym}"))?;
        Ok(())
    }

    pub async fn move_mouse_absolute(&self, x: f64, y: f64) -> anyhow::Result<()> {
        self.remote
            .notify_pointer_motion_absolute(&self.session, self.stream.pipe_wire_node_id(), x, y)
            .await
            .with_context(|| format!("Failed to move pointer to ({x}, {y})"))?;
        Ok(())
    }
}
