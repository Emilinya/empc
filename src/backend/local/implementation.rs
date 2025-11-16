use std::io::ErrorKind;

use dioxus::prelude::*;
use tokio::fs;

use super::DirEntry;

pub async fn list_files_impl(directory: String) -> Result<Vec<DirEntry>, HttpError> {
    let mut iterator = match fs::read_dir(&directory).await {
        Ok(iterator) => iterator,
        Err(err) => {
            let status = match err.kind() {
                ErrorKind::NotFound => StatusCode::NOT_FOUND,
                ErrorKind::PermissionDenied => StatusCode::FORBIDDEN,
                ErrorKind::NotADirectory => StatusCode::BAD_REQUEST,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            return Err(HttpError::new(status, err.to_string()));
        }
    };

    let mut entries = Vec::new();
    loop {
        let entry = match iterator.next_entry().await {
            Ok(Some(entry)) => entry,
            Err(err) => {
                warn!("Failed to read dir entry: {}", err);
                continue;
            }
            Ok(None) => break,
        };

        let file_type = match entry.file_type().await {
            Ok(file_type) => file_type.into(),
            Err(err) => {
                warn!("Failed to read file type of {:?} {}", entry.path(), err);
                continue;
            }
        };

        entries.push(DirEntry {
            path: entry.path(),
            file_name: entry.file_name().to_string_lossy().into_owned(),
            file_type,
        })
    }

    Ok(entries)
}
