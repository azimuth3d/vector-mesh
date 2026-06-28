use crate::error::{AppError, Result};
use qdrant_client::Qdrant;

pub fn init_qdrant(uri: &str) -> Result<Qdrant> {
    let client = Qdrant::from_url(uri)
        .build()
        .map_err(|e| AppError::Qdrant(e.to_string()))?;
    Ok(client)
}
