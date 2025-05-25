use std::time::Duration;

use anyhow::{Result, bail};
use minio::s3::{args::PutObjectArgs, client::Client};

#[derive(Default, Clone)]
pub struct AdditionalOptions {
    pub cookie: Option<String>,
    pub mime_type: Option<String>,
}

impl AdditionalOptions {
    pub fn new(cookie: String, mime_type: String) -> Self {
        Self {
            cookie: Some(cookie),
            mime_type: Some(mime_type),
        }
    }
    pub fn cookie(cookie: String) -> Self {
        Self {
            cookie: Some(cookie),
            ..Default::default()
        }
    }
    pub fn mime_type(mime_type: String) -> Self {
        Self {
            mime_type: Some(mime_type),
            ..Default::default()
        }
    }
}

#[async_recursion::async_recursion]
async fn download_file(url: &str, cookie: Option<String>) -> Result<bytes::Bytes> {
    if url.contains("-small.") {
        tracing::info!("URL contains -small, replacing with -large");
        // try download_file_to_minio with the new url, if it fails, try the original url
        let result = download_file(&url.replace("-small.", "-large."), cookie.clone()).await;
        if let Ok(result) = result {
            return Ok(result);
        } else {
            tracing::info!(
                "Failed to download from {}: {:?}",
                url.replace("-small.", "-large."),
                result
            );
        }
    };
    let client = reqwest::Client::new();
    let mut request: reqwest::RequestBuilder = client.get(url);
    if let Some(cookie) = cookie {
        request = request.header("Cookie", cookie);
    }

    let response = request.send().await.expect("Failed to send request");
    if !response.status().is_success() {
        bail!("Failed to download file");
    }
    let data = response.bytes().await?;
    Ok(data)
}

pub async fn download_file_to_minio(
    url: &str,
    bucket: &str,
    object: &str,
    storage: &Client,
    additional_options: AdditionalOptions,
) {
    tracing::info!("Downloading file from {} to {}/{}", url, bucket, object);
    let data = download_file(url, additional_options.cookie.clone())
        .await
        .expect("Failed to download file");
    let data_len = data.len();
    let mut data_stream = std::io::Cursor::new(data);
    let mut upload_args =
        PutObjectArgs::new(bucket, object, &mut data_stream, Some(data_len), None)
            .expect("Failed to create PutObjectArgs");
    let mut original_mime_type = match object.rsplit_once(".") {
        Some((_, ext)) => match ext {
            "mp4" => "video/mp4",
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "webp" => "image/webp",
            "html" => "text/html",
            "css" => "text/css",
            _ => "application/octet-stream",
        },
        None => "application/octet-stream",
    }
    .to_string();
    if let Some(mime_type) = additional_options.mime_type {
        original_mime_type = mime_type.to_string();
    }
    upload_args.content_type = &original_mime_type;
    storage
        .put_object(&mut upload_args)
        .await
        .expect("Failed to upload file to MinIO");
}

pub async fn download_m3u8_as_mp4_to_minio(
    url: &str,
    bucket: &str,
    object: &str,
    token: &str,
    storage: &Client,
) -> Result<()> {
    println!("Starting movie download...");
    let tempfile = format!("/tmp/{}", object.replace("/", "_"));
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-headers")
        .arg(format!("__token__: {}", token))
        .arg("-i")
        .arg(url)
        .arg("-c")
        .arg("copy")
        .arg(&tempfile);
    println!("ffmpeg command: {:?}", cmd);
    // Use ffmpeg to download the m3u8 file and convert it to mp4
    let output = cmd.output().expect("Failed to execute ffmpeg");

    if !output.status.success() {
        eprintln!(
            "ffmpeg failed with status: {}",
            output.status.code().unwrap_or(-1)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("stderr: {}", stderr);
        eprintln!("stdout: {}", stdout);
        panic!();
    } else {
    }
    println!("Movie download completed.");
    tokio::time::sleep(Duration::from_secs(20)).await;
    // Upload the mp4 file to MinIO
    let mut file = std::fs::File::open(&tempfile)?;
    let file_path = std::path::Path::new(&tempfile);
    let file_len = file_path.metadata()?.len();
    if file_len == 0 {
        panic!("File is empty");
    }
    println!("Uploading movie to MinIO...");
    storage
        .put_object(
            &mut PutObjectArgs::new(bucket, object, &mut file, Some(file_len as usize), None)
                .expect("Failed to create PutObjectArgs"),
        )
        .await
        .expect("Failed to upload file to MinIO");
    println!("Movie uploaded to MinIO.");
    Ok(())
}
