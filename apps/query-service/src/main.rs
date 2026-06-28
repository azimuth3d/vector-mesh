use shared_lib::error::Result;
use tracing::info;

mod domain;
mod use_cases;
mod infrastructure;
mod adapters;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Query service started");

    let app = adapters::router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
