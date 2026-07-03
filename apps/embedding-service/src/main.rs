use qdrant_client;
use qdrant_client::Payload;
use serde::Deserialize;
use shared_lib::error::Result;
use tokio::time::{Duration, sleep};
use tracing::{info, instrument};

#[derive(Deserialize, Debug)]
struct QueueMessage {
    id: String,
    text: String,
}

#[instrument(skip(qdrant_client))]
async fn process_message(qdrant_client: &qdrant_client::Qdrant, msg: QueueMessage) -> Result<()> {
    let chunks: Vec<String> = text_chunker(&msg.text);

    let embedding_req = serde_json::json!({
        "model": "text-embedding-3-small",
        "input": chunks.clone()
    });

    let embeddings: Vec<Vec<f32>> = reqwest::Client::new()
        .post("http://embedding-model:8080/embed")
        .json(&embedding_req)
        .send()
        .await
        .map_err(|e| shared_lib::error::AppError::Internal(e.to_string()))?
        .json::<Vec<Vec<f32>>>()
        .await
        .map_err(|e| shared_lib::error::AppError::Internal(e.to_string()))?;

    for (chunk, embedding) in chunks.into_iter().zip(embeddings) {
        let payload: Payload = Payload::try_from(serde_json::json!({
            "text": chunk
        }))
        .map_err(|e| shared_lib::error::AppError::Internal(e.to_string()))?;

        let point = qdrant_client::qdrant::PointStruct::new(msg.id.clone(), embedding, payload);
        qdrant_client
            .upsert_points(qdrant_client::qdrant::UpsertPointsBuilder::new(
                "documents",
                vec![point],
            ))
            .await
            .map_err(|e| shared_lib::error::AppError::Qdrant(e.to_string()))?;
    }

    Ok(())
}

fn text_chunker(text: &str) -> Vec<String> {
    text.split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(100)
        .map(|chunk| chunk.join(" "))
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!("Embedding service started");

    let redis_client = shared_lib::db::init_redis("redis://redis:6379")?;
    let qdrant_client = shared_lib::clients::init_qdrant("http://qdrant:6334")?;

    loop {
        let mut conn = redis_client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| shared_lib::error::AppError::Redis(e.to_string()))?;

        let raw_msg: Option<String> = redis::cmd("RPOP")
            .arg("ingestion_queue")
            .query_async(&mut conn)
            .await
            .map_err(|e| shared_lib::error::AppError::Redis(e.to_string()))?;

        match raw_msg {
            Some(raw) => {
                let msg: QueueMessage = serde_json::from_str(&raw)
                    .map_err(|e| shared_lib::error::AppError::Internal(e.to_string()))?;

                if let Err(e) = process_message(&qdrant_client, msg).await {
                    tracing::error!("Failed to process message: {}", e);
                }
            }
            None => sleep(Duration::from_millis(500)).await,
        }
    }
}
