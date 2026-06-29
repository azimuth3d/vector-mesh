use axum::{Json, routing::post, Router};
use shared_lib::error::{AppError, Result};
use shared_lib::domain::{QueryRequest, QueryResponse};
use crate::use_cases::handle_query;

pub fn router() -> Router {
    Router::new()
        .route("/query", post(query_handler))
}

async fn query_handler(
    Json(payload): Json<QueryRequest>,
) -> Result<Json<QueryResponse>> {
    let response = handle_query(payload).await?;
    Ok(response)
}
