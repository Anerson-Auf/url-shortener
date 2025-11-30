mod service;
use service::{types::Service, migrator};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    migrator::migrate().await?;
    let service = Service::new().await?;
    service.start().await?;
    Ok(())
}
