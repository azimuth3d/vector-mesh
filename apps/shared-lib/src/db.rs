use crate::error::{AppError, Result};
use scylla::client::session::Session;
use scylla::client::session_builder::SessionBuilder;
use std::sync::Arc;

pub type RedisClient = redis::Client;

pub async fn init_scylla(uri: &str) -> Result<Arc<Session>> {
    let session = SessionBuilder::new()
        .known_node(uri)
        .build()
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
    Ok(Arc::new(session))
}

pub fn init_redis(uri: &str) -> Result<RedisClient> {
    let client = redis::Client::open(uri)
        .map_err(|e| AppError::Redis(e.to_string()))?;
    Ok(client)
}
