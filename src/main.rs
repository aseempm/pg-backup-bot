use anyhow::Result;
use dotenv::dotenv;

mod postgres;
mod telegram;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let backup_path = postgres::backup().await?;
    telegram::send(&backup_path).await?;

    Ok(())
}
