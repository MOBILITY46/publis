use crate::s3::{BucketPolicy, Client};
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub async fn upload_all(root: &str, bucket: &str, dry: bool, max_age: u16) -> Result<(), String> {
    let client = Client::new()?;
    let mut entries = Vec::new();

    walk_bundle(Path::new(root), &mut entries).map_err(|e| format!("IO error: {}", e))?;

    let entries = entries.into_iter();

    for entry in entries {
        client.upload(entry, bucket, dry, max_age).await?;
    }

    Ok(())
}

pub async fn add_bucket_policy(bucket: &str) -> Result<BucketPolicy, String> {
    let client = Client::new()?;
    let policy_opt = client.get_policy(bucket).await?;

    match policy_opt {
        Some(p) => {
            // TODO: This means that there is existing policies.
            // If it does NOT contain the website policy, add and upload it.
            // We need to figure out how it is supposed to look with multiple

            let policy = serde_json::from_str::<BucketPolicy>(&p)
                .map_err(|e| format!("Error while deserializing policy: {}", e))?;
            Ok(policy)
        }
        None => {
            let policy = client.add_website_policy(bucket).await?;
            Ok(policy)
        }
    }
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
