mod robot;

use dioxus::{
    fullstack::{response::IntoResponse, ClientResponse, FromResponse},
    prelude::*,
};
use jpeg_encoder::ColorType;
use pipewire::spa::param::video::VideoFormat;
use robot::Robot;
use std::future::Future;
use std::{
    io::Write,
    pin::Pin,
    sync::{Arc, LazyLock, Mutex},
    task,
    thread::{self, JoinHandle},
    time::Duration,
};
use tokio::{sync::broadcast, time::sleep};

#[derive(Clone)]
struct JpegFrame(Arc<Vec<u8>>);

impl From<JpegFrame> for dioxus_server::Bytes {
    fn from(value: JpegFrame) -> Self {
        let len = value.0.len();
        let mut bytes = Vec::<u8>::new();
        bytes.reserve(len + 100);
        write!(&mut bytes, "--EMPC_FRAME_BOUNDARY\r\n").expect("write should never fail");
        write!(&mut bytes, "Content-Type: image/jpeg\r\n").expect("write should never fail");
        write!(&mut bytes, "Content-Length: {len}\r\n").expect("write should never fail");
        write!(&mut bytes, "\r\n").expect("write should never fail");
        bytes.extend_from_slice(&value.0);
        dioxus_server::Bytes::from(bytes)
    }
}

struct Context {
    _thread: JoinHandle<()>,
    _robot: Arc<Mutex<Option<Robot>>>,
    super_receiver: broadcast::Receiver<JpegFrame>,
}

fn worker_thread(frame_tx: broadcast::Sender<JpegFrame>, robot: Arc<Mutex<Option<Robot>>>) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let mut rob = robot::Robot::new().await.unwrap();
        let rx = rob.start_streaming().await.unwrap();
        *robot.lock().unwrap() = Some(rob);

        for frame in rx {
            if frame_tx.receiver_count() == 1 {
                // We don't have anyone waiting for the frame,
                // so we sleep a second to apply backpressure
                // then don't encode it
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            let color_type = match frame.format.format() {
                VideoFormat::BGRx => ColorType::Bgra,
                VideoFormat::BGRA => ColorType::Bgra,
                VideoFormat::RGBx => ColorType::Rgba,
                VideoFormat::RGBA => ColorType::Rgba,
                VideoFormat::RGB => ColorType::Rgb,
                VideoFormat::BGR => ColorType::Bgr,
                f => {
                    warn!("Unknown pixel format: {:?}", f);
                    continue;
                }
            };

            let mut encoded = Vec::<u8>::new();
            let enc = jpeg_encoder::Encoder::new(&mut encoded, 70);
            let size = frame.format.size();
            if let Err(err) = enc.encode(
                &frame.buffer,
                size.width as u16,
                size.height as u16,
                color_type,
            ) {
                warn!("Encoding failed: {err}");
                continue;
            };

            let frame = Arc::new(encoded);
            if let Err(err) = frame_tx.send(JpegFrame(frame)) {
                info!("Worker thread exiting: {err}");
                break;
            }
        }
    });
}

impl Context {
    pub fn new() -> Self {
        let robot = Arc::new(Mutex::<Option<Robot>>::new(None));
        let (tx, rx) = broadcast::channel::<JpegFrame>(1);

        let thread = {
            let robot = robot.clone();
            thread::spawn(move || {
                worker_thread(tx, robot);
            })
        };

        Self {
            _robot: robot,
            _thread: thread,
            super_receiver: rx,
        }
    }
}

static CONTEXT: LazyLock<Context> = LazyLock::new(|| Context::new());

struct JpegStream(broadcast::Receiver<JpegFrame>);

impl futures::Stream for JpegStream {
    type Item = anyhow::Result<JpegFrame>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        match std::pin::pin!(self.0.recv()).poll(cx) {
            task::Poll::Ready(Ok(x)) => task::Poll::Ready(Some(Ok(x))),
            task::Poll::Ready(Err(_)) => task::Poll::Ready(None),
            task::Poll::Pending => task::Poll::Pending,
        }
    }
}

pub struct ScreencastResponse(broadcast::Receiver<JpegFrame>);

impl IntoResponse for ScreencastResponse {
    fn into_response(self) -> axum::response::Response {
        use axum::body::Body;

        let stream = JpegStream(self.0);
        let mut res = axum::response::Response::new(Body::from_stream(stream));

        let content_type = "multipart/x-mixed-replace;boundary=EMPC_FRAME_BOUNDARY";
        res.headers_mut().insert(
            "Content-Type",
            content_type
                .parse()
                .expect("content_type should be a well-formed header value"),
        );

        res
    }
}

impl FromResponse for ScreencastResponse {
    fn from_response(_res: ClientResponse) -> impl Future<Output = Result<Self, ServerFnError>> {
        async {
            panic!("FromResponse is not to be used directly from client code");
        }
    }
}

pub async fn screencast() -> Result<ScreencastResponse> {
    let rx = CONTEXT.super_receiver.resubscribe();
    Ok(ScreencastResponse(rx))
}
