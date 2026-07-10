use shared_lib::error::Result;
use tracing::info;

mod domain;
mod use_cases;
mod infrastructure;
mod adapters;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let provider = std::env::var("LLM_PROVIDER").unwrap_or_else(|_| "ollama".to_string());
    info!("Query service started with LLM provider: {}", provider);

    let app = adapters::router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
