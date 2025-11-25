use dioxus::fullstack::{PostcardEncoding, WebSocketOptions, Websocket};
use dioxus::prelude::*;

#[cfg(all(target_os = "linux", feature = "server"))]
pub mod linux;
#[cfg(all(target_os = "linux", feature = "server"))]
pub use linux as implementation;

#[cfg(all(not(target_os = "linux"), feature = "server"))]
pub mod dummy;
#[cfg(all(not(target_os = "linux"), feature = "server"))]
pub use dummy as implementation;

use crate::frontend::remote::{Interaction, RelativePosition};

#[cfg(feature = "server")]
#[get("/api/remote/screencast")]
pub async fn screencast() -> Result<implementation::ScreencastResponse, HttpError> {
    implementation::screencast().await
}

#[get("/api/remote/interaction")]
pub async fn interaction(
    options: WebSocketOptions,
) -> Result<Websocket<Interaction, RelativePosition, PostcardEncoding>, HttpError> {
    Ok(options.on_upgrade(|mut socket| async move {
        loop {
            let message = match socket.recv().await {
                Ok(message) => message,
                Err(err) => {
                    error!("socket.recv() returned an error: {err}");
                    return;
                }
            };

            info!("Got message: {:?}", message);

            if let Interaction::Position(position) = message
                && let Err(err) = socket.send(position).await
            {
                warn!("Failed to send message: {}", err);
            }
        }
    }))
}
