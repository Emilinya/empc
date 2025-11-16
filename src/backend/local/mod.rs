#[cfg(feature = "server")]
mod implementation;

use std::path::PathBuf;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Unknown,
}

impl From<std::fs::FileType> for FileType {
    fn from(value: std::fs::FileType) -> Self {
        if value.is_file() {
            Self::File
        } else if value.is_dir() {
            Self::Directory
        } else if value.is_symlink() {
            Self::Symlink
        } else {
            debug!(
                "Got filetype that is not file, directory, or symlink: {:?}",
                value
            );
            Self::Unknown
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirEntry {
    pub path: PathBuf,
    pub file_name: String,
    pub file_type: FileType,
}

#[get("/api/local/files?directory")]
pub async fn list_files(directory: String) -> Result<Vec<DirEntry>, HttpError> {
    implementation::list_files_impl(directory).await
}
