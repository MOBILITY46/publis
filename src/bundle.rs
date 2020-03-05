use crate::s3::Client;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub async fn upload_all(root: &str, bucket: &str) -> Result<(), String> {
    let client = Client::new()?;
    let mut entries = Vec::new();

    walk_bundle(Path::new(root), &mut entries).map_err(|e| format!("IO error: {}", e))?;

    let entries = entries.into_iter();

    for entry in entries {
        client.upload(entry, bucket).await?;
    }

    Ok(())
}

fn walk_bundle(dir: &Path, entries: &mut Vec<DirEntry>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_bundle(&path, entries)?;
            } else {
                entries.push(entry);
            }
        }
    }
    Ok(())
}
