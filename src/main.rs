use anyhow::{Result, anyhow};
use bytes::Bytes;
use dotenv::dotenv;
use futures_util::TryStreamExt;
use reqwest::{Client, multipart};
use std::{env, process::Stdio};
use tokio::{fs::File, process::Command};
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let backup_path = pg_backup().await?;
    send_to_telegram(&backup_path).await?;

    Ok(())
}

async fn pg_backup() -> Result<String> {
    let host = env::var("DATABASE_HOST")?;
    let port = env::var("DATABASE_PORT")?;
    let db = env::var("DATABASE_NAME")?;
    let user = env::var("DATABASE_USER")?;
    let output_path = env::var("DATABASE_BACKUP_PATH")?;

    let mut cmd = Command::new("pg_dump");
    cmd.args([
        "-h",
        &host,
        "-p",
        &port,
        "-U",
        &user,
        "-F",
        "c", // custom format, or "p" for plain SQL
        "-f",
        &output_path,
        &db,
    ]);

    cmd.env(
        "PGPASSWORD",
        env::var("DATABASE_PASSWORD").unwrap_or_default(),
    );

    let status = cmd.stdout(Stdio::null()).status().await?;
    if !status.success() {
        return Err(anyhow!("pg_dump failed with status: {:?}", status));
    }

    Ok(output_path)
}

async fn send_to_telegram(file_path: &str) -> Result<()> {
    let bot_token = env::var("TELEGRAM_BOT_TOKEN")?;
    let chat_id = env::var("TELEGRAM_CHAT_ID")?;

    let url = format!("https://api.telegram.org/bot{}/sendDocument", bot_token);

    let file = File::open(file_path).await?;
    let metadata = file.metadata().await?;
    let file_size = metadata.len();

    let stream = FramedRead::new(file, BytesCodec::new()).map_ok(|b| Bytes::from(b.freeze()));
    let body = reqwest::Body::wrap_stream(stream);

    let file_part = multipart::Part::stream_with_length(body, file_size)
        .file_name("backup.sql")
        .mime_str("application/octet-stream")?;

    let form = multipart::Form::new()
        .text("chat_id", chat_id)
        .part("document", file_part);

    let client = Client::new();
    let response = client.post(&url).multipart(form).send().await?;

    if response.status().is_success() {
        println!("✅ Backup sent to Telegram!");
    } else {
        println!("❌ Telegram send error: {}", response.text().await?);
    }

    Ok(())
}
