use bytes::Bytes;
use std::env;

use anyhow::Result;
use futures_util::TryStreamExt;
use reqwest::{Client, multipart};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

pub async fn send(file_path: &str) -> Result<()> {
    let discord_hook = env::var("DISCORD_HOOK")?;

    let file = File::open(file_path).await?;
    let metadata = file.metadata().await?;
    let file_size = metadata.len();

    let stream = FramedRead::new(file, BytesCodec::new()).map_ok(|b| Bytes::from(b.freeze()));
    let body = reqwest::Body::wrap_stream(stream);

    let file_part = multipart::Part::stream_with_length(body, file_size)
        .file_name("backup.sql")
        .mime_str("application/octet-stream")?;

    let form = multipart::Form::new().part("document", file_part);

    let client = Client::new();
    let response = client.post(&discord_hook).multipart(form).send().await?;

    if response.status().is_success() {
        println!("✅ Backup sent to Discord!");
    } else {
        println!("❌ Discord send error: {}", response.text().await?);
    }

    Ok(())
}
