use dioxus::prelude::*;

#[cfg(all(target_os = "linux", feature = "server"))]
pub mod linux;
#[cfg(all(target_os = "linux", feature = "server"))]
pub use linux as implementation;

#[cfg(all(not(target_os = "linux"), feature = "server"))]
pub mod dummy;
#[cfg(all(not(target_os = "linux"), feature = "server"))]
pub use dummy as implementation;

#[cfg(feature = "server")]
#[get("/api/remote/screencast")]
pub async fn screencast() -> Result<implementation::ScreencastResponse> {
    implementation::screencast().await
}
