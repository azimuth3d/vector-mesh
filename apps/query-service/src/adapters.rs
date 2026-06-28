use axum::{extract::State, Json, routing::post, Router};
use axum::http::StatusCode;
use shared_lib::error::{AppError, Result};
use shared_lib::domain::{QueryRequest, QueryResponse};
use crate::use_cases::handle_query;

pub fn router() -> Router {
    Router::new()
        .route("/query", post(query_handler))
}

async fn query_handler(
    State(_): State<()>,
    Json(payload): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, (StatusCode, Json<AppError>)> {
    let response = handle_query(payload).await?;
    Ok(Json(response))
}
