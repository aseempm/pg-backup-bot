use anyhow::{Ok, Result};
use dotenv::dotenv;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    send_to_telegram().await?;

    Ok(())
}

async fn send_to_telegram() -> Result<()> {
    let chat_id = std::env::var("TELEGRAM_CHAT_ID")?;
    let bot_token = std::env::var("TELEGRAM_BOT_TOKEN")?;

    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);

    let client = Client::new();
    let res = client
        .post(&url)
        .json(&json!({
            "chat_id":chat_id,
             "text": "hi"
        }))
        .send()
        .await?;

    if res.status().is_success() {
        println!("Message sent successfully");
    } else {
        let text = res.text().await?;
        println!("Failed to send message: {}", text);
    }

    Ok(())
}
