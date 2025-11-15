use dioxus::prelude::*;
use tokio::fs;

use super::DirEntry;

pub async fn list_files_impl(directory: String) -> Result<Vec<DirEntry>> {
    let mut entries = Vec::new();
    let mut iterator = fs::read_dir(&directory).await?;
    loop {
        let entry = match iterator.next_entry().await {
            Ok(Some(entry)) => entry,
            Err(err) => {
                eprintln!("Failed to read dir entry: {}", err);
                continue;
            }
            Ok(None) => break,
        };

        let file_type = match entry.file_type().await {
            Ok(file_type) => file_type.into(),
            Err(err) => {
                eprintln!("Failed to read file type of {:?} {}", entry.path(), err);
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
