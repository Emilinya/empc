#[cfg(feature = "server")]
mod implementation;

use std::path::PathBuf;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FileType {
    File,
    Unknown,
    Directory,
}

impl From<std::fs::FileType> for FileType {
    fn from(value: std::fs::FileType) -> Self {
        if value.is_file() {
            Self::File
        } else if value.is_dir() {
            Self::Directory
        } else {
            eprintln!("Got filetype that is not file or directory: {:?}", value);
            Self::Unknown
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirEntry {
    path: PathBuf,
    file_name: String,
    file_type: FileType,
}

#[get("/api/local/files?directory")]
pub async fn list_files(directory: String) -> Result<Vec<DirEntry>, ServerFnError> {
    implementation::list_files_impl(directory)
        .await
        .map_err(ServerFnError::new)
}
