use anyhow::Result;
use dotenv::dotenv;

mod discord;
mod postgres;
// mod telegram;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let backup_path = postgres::backup().await?;
    discord::send(&backup_path).await?;
    // telegram::send(&backup_path).await?;

    Ok(())
}
