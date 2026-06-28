use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    #[error("Redis error: {0}")]
    Redis(String),
    #[error("Qdrant error: {0}")]
    Qdrant(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
