use dioxus::prelude::*;
use dioxus_fullstack::response::Response;

#[get("/api/shutdown/")]
pub async fn shutdown() -> Result<Response> {
    use axum::{body::Body, http::header::LOCATION};

    // Redirect to empc.emilie.moe, which can be used to restart empc
    let response = Response::builder()
        .status(StatusCode::PERMANENT_REDIRECT)
        .header(LOCATION, "http://empc.emilie.moe/")
        .body(Body::empty())?;

    // Shut down in a separate task so we have enough time to redirect before powering off
    #[cfg(target_os = "linux")]
    tokio::spawn(async {
        let mut handle = match tokio::process::Command::new("poweroff").spawn() {
            Ok(handle) => handle,
            Err(err) => {
                error!("Failed to spawn poweroff command: {}", err);
                return;
            }
        };

        if let Err(err) = handle.wait().await {
            error!("Failed to run poweroff command: {}", err);
        }
    });

    Ok(response)
}
