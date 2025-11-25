use dioxus::{
    html::geometry::{Pixels, euclid::Size2D},
    prelude::*,
};

use super::{EqWebsocket, Interaction, MouseButton, RelativePosition, WheelDelta};

static CURSOR: Asset = asset!("/assets/cursor.png");

#[component]
pub fn Screen(socket: EqWebsocket) -> Element {
    let mut screen_size = use_signal(|| Option::<Size2D<f64, Pixels>>::None);

    // update cursor position based on data we get from the backend
    let mut cursor_top = use_signal(|| "0px".to_owned());
    let mut cursor_left = use_signal(|| "0px".to_owned());
    use_future(move || async move {
        loop {
            let message = match socket.recv().await {
                Ok(message) => message,
                Err(err) => {
                    warn!("socket.recv() returned an error: {err}");
                    return;
                }
            };

            let Some(size) = screen_size() else {
                warn!("Screen size not set, ignoring mouse message from backend");
                continue;
            };
            let absolute_position = message.into_absolute_position(size);
            *cursor_left.write() = format!("{}px", absolute_position.x);
            *cursor_top.write() = format!("{}px", absolute_position.y);
        }
    });

    rsx! {
        div {
            id: "screen",
            onmounted: move |event| async move {
                match event.data.get_client_rect().await {
                    Ok(rect) => {
                        let size = rect.size;
                        info!("Setting screen size to ({:.3}, {:.3})", size.width, size.height);
                        *screen_size.write() = Some(size);
                    }
                    Err(err) => warn!("Failed to get client rect of event {:?}: {}", event, err),
                }
            },
            onresize: move |event| {
                match event.data.get_content_box_size() {
                    Ok(size) => {
                        info!("Setting screen size to ({:.3}, {:.3})", size.width, size.height);
                        *screen_size.write() = Some(size);
                    }
                    Err(err) => {
                        warn!("Failed to get content box size of event {:?}: {}", event, err)
                    }
                }
            },
            onmousemove: move |event| async move {
                let Some(size) = screen_size() else {
                    warn!("Screen size not set, ignoring onmousedown");
                    return;
                };

                let position = RelativePosition::from_absolute_position(
                    event.data.coordinates().element(),
                    size,
                );
                if let Err(err) = socket.send(Interaction::Position(position)).await {
                    warn!("Failed to send {:?} to socket: {}", event, err);
                }
            },
            onmousedown: move |event| async move {
                if let Some(button) = event.data.trigger_button()
                    && let Ok(button) = MouseButton::try_from(button)
                    && let Err(err) = socket.send(Interaction::MouseDown(button)).await
                {
                    warn!("Failed to send {:?} to socket: {}", event, err);
                }
            },
            onmouseup: move |event| async move {
                if let Some(button) = event.data.trigger_button()
                    && let Ok(button) = MouseButton::try_from(button)
                    && let Err(err) = socket.send(Interaction::MouseDown(button)).await
                {
                    warn!("Failed to send {:?} to socket: {}", event, err);
                }
            },
            onwheel: move |event| async move {
                let pixel_distance = match event.data.delta() {
                    WheelDelta::Pixels(vector) => vector.y,
                    WheelDelta::Lines(vector) => vector.y * 100.0 / 6.0,
                    WheelDelta::Pages(_vector) => {
                        warn!("Got a wheel delta in pages! What does that mean?");
                        return;
                    }
                };
                if pixel_distance != 0.0
                    && let Err(err) = socket
                        .send(Interaction::Scroll(pixel_distance as f32))
                        .await
                {
                    warn!("Failed to send {:?} to socket: {}", event, err);
                }
            },
            // ontouchmove: |event| info!("{:?}", event.data),
            // ontouchstart: |event| info!("{:?}", event.data),
            // ontouchend: |event| info!("{:?}", event.data),
            img { id: "screen-img", src: "/api/remote/screencast" }
            img {
                id: "screen-cursor",
                src: CURSOR,
                top: cursor_top,
                left: cursor_left,
            }
        }
    }
}
