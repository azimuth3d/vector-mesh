use shared_lib::{AppError, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Ingestion service started");
    Ok(())
}
