use axum::{Router, extract::State, http::StatusCode, response::Json, routing::post};
use redis::AsyncCommands;
use shared_lib::{
    db::{RedisClient, init_redis},
    domain::IngestionMessage,
    error::Result,
};

#[derive(Clone)]
struct AppState {
    redis_client: RedisClient,
}

async fn ingest_handler(
    State(state): State<AppState>,
    Json(msg): Json<IngestionMessage>,
) -> Result<(StatusCode, Json<serde_json::Value>)> {
    let mut con = state
        .redis_client
        .get_async_connection()
        .await
        .map_err(|e| shared_lib::AppError::Redis(e.to_string()))?;

    let payload =
        serde_json::to_string(&msg).map_err(|e| shared_lib::AppError::Internal(e.to_string()))?;

    let _: () = con
        .rpush("documents:queue", &payload)
        .await
        .map_err(|e| shared_lib::AppError::Redis(e.to_string()))?;

    tracing::info!("Document {} pushed to queue", msg.id);

    Ok((
        StatusCode::ACCEPTED,
        Json(serde_json::json!({ "status": "queued", "id": msg.id })),
    ))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Ingestion service started");

    let redis_uri =
        std::env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string());
    let redis_client = init_redis(&redis_uri)?;

    let state = AppState { redis_client };
    let app = Router::new()
        .route("/ingest", post(ingest_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Listening on 0.0.0.0:8080");
    axum::serve(listener, app).await?;

    Ok(())
}
