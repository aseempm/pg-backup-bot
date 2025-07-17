use std::{
    env,
    process::{Command, Stdio},
};

use anyhow::{Result, anyhow};

pub async fn pg_backup() -> Result<String> {
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

    let status = cmd.stdout(Stdio::null()).status()?;
    if !status.success() {
        return Err(anyhow!("pg_dump failed with status: {:?}", status));
    }

    Ok(output_path)
}
