use thiserror::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response, Json};
use serde_json::json;

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

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::Database(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Redis(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Qdrant(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}
