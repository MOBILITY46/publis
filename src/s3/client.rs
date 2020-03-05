use super::Config;
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::request::HttpClient;
use rusoto_s3::{S3Client, S3};
use std::fs::{self, File};
use std::io::Read;

pub struct Client {
    s3: S3Client,
}

impl Client {
    pub fn new() -> Result<Self, String> {
        let config = Config::from_env();
        let region = config.region.clone();

        let http_client = HttpClient::new().expect("Failed to create HTTP client");

        let s3_client = S3Client::new_with(
            http_client,
            EnvironmentProvider::default(),
            region.parse().expect("Unknown AWS region"),
        );

        Ok(Self { s3: s3_client })
    }

    pub async fn upload(&self, entry: std::fs::DirEntry, bucket: &str) -> Result<(), String> {
        let mime = mime_guess::from_path(entry.path());
        let mut file = File::open(entry.path()).map_err(|e| format!("File open error: {}", e))?;

        let file_name = entry
            .file_name()
            .into_string()
            .expect("Could not read filename");

        println!("Uploading entry: {}", file_name);

        let data = self
            .read_bytes(&mut file)
            .map_err(|e| format!("File read error: {}", e))?;

        let metadata =
            fs::metadata(entry.path()).map_err(|e| format!("File metadata error: {}", e))?;

        let request = rusoto_s3::PutObjectRequest {
            bucket: bucket.into(),
            content_type: mime.first().map(|m| m.to_string()),
            body: Some(data.into()),
            key: file_name,
            content_length: Some(metadata.len() as i64),
            ..rusoto_s3::PutObjectRequest::default()
        };

        self.s3
            .put_object(request)
            .await
            .map_err(|e| format!("S3 upload error: {}", e))?;

        Ok(())
    }

    fn read_bytes(&self, file: &mut std::fs::File) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Ok(data)
    }
}
