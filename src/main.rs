use anyhow::Result;
use dotenv::dotenv;

mod db;
mod telegram;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let backup_path = db::pg_backup().await?;
    telegram::send_to_telegram(&backup_path).await?;

    Ok(())
}
