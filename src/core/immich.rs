use anyhow::{Context, Result};
use reqwest::{Body, multipart};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio_util::codec::{BytesCodec, FramedRead};

use chrono::{SecondsFormat, Utc};

pub async fn upload_asst(server_url: &str, api_key: &str, file_path: PathBuf) -> Result<()> {
    tracing::debug!("Starting upload for file: {:?}", file_path);

    let client = reqwest::Client::new();
    let url = format!("{}/api/assets", server_url.trim_end_matches("/"));
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .context("Invalid file name")?;

    let mime_type = get_mime_type(&file_path).await?;
    tracing::debug!("MIME type for {}: {}", file_name, mime_type);

    let file_metadata = std::fs::metadata(&file_path).context("Failed to read file metadata")?;
    let file_size = file_metadata.len();

    let file = File::open(&file_path)
        .await
        .context("Failed to open file")?;
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);

    let created_at: chrono::DateTime<Utc> = file_metadata
        .created()
        .unwrap_or(std::time::SystemTime::now())
        .into();
    let modified_at: chrono::DateTime<Utc> = file_metadata
        .modified()
        .unwrap_or(std::time::SystemTime::now())
        .into();
    let created_at_string = created_at.to_rfc3339_opts(SecondsFormat::Millis, true);
    let modified_at_string = modified_at.to_rfc3339_opts(SecondsFormat::Millis, true);
    let device_asset_id = format!("{}-{}", file_name, file_size);

    let file_part = multipart::Part::stream(body)
        .file_name(file_name.to_string())
        .mime_str(&mime_type)
        .context("Failed to set MIME type")?;

    let form = multipart::Form::new()
        .text("deviceAssetId", device_asset_id)
        .text("deviceId", "rimmich-desktop")
        .text("fileCreatedAt", created_at_string)
        .text("fileModifiedAt", modified_at_string)
        .text("isFavorite", "false")
        .text("isReadOnly", "false")
        .text("filename", file_name.to_string())
        .part("assetData", file_part);

    let resp = client
        .post(url)
        .header("x-api-key", api_key)
        .multipart(form)
        .send()
        .await
        .context("Failed to send upload request")?;

    let status = resp.status();
    if status.is_success() {
        tracing::info!("File uploaded successfully: {}", file_name);
        Ok(())
    } else {
        let error_text = resp
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error messages".to_string());
        tracing::error!(
            "File upload failed for {}: status {}, error: {}",
            file_name,
            status,
            error_text
        );
        anyhow::bail!("Upload failed with status {}: {}", status, error_text)
    }
}

async fn get_mime_type(file_path: &PathBuf) -> Result<String> {
    let mut buffer = [0; 512];
    let mut file_for_detection = File::open(&file_path)
        .await
        .context("Failed to open file for MIME detection")?;
    let bytes_read = AsyncReadExt::read(&mut file_for_detection, &mut buffer)
        .await
        .context("Failed to read file for MIME detection")?;
    let mime_type = infer::get(&buffer[..bytes_read])
        .map(|kind| kind.mime_type())
        .unwrap_or_else(|| {
            tracing::warn!(
                "Unable to detect MIME type for {:?}, using default",
                file_path
            );
            "application/octet-stream"
        })
        .to_string();

    Ok(mime_type)
}
