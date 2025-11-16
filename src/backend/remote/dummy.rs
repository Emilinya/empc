use dioxus::prelude::*;

pub type ScreencastResponse = ();

pub async fn screencast() -> Result<ScreencastResponse, HttpError> {
    Err(HttpError {
        status: StatusCode::NOT_IMPLEMENTED,
        message: None,
    })
}
