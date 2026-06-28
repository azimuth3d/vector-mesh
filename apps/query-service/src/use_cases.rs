use shared_lib::domain::{QueryRequest, QueryResponse};
use shared_lib::error::Result;
use crate::infrastructure::{query_qdrant, call_ollama, save_to_scylla};

pub async fn handle_query(req: QueryRequest) -> Result<QueryResponse> {
    let context = query_qdrant(&req.question).await?;
    let prompt = format!(
        "Context: {}\nHistory: {}\nQuestion: {}",
        context.join("\n"),
        req.chat_history.join("\n"),
        req.question
    );
    let answer = call_ollama(&prompt).await?;
    save_to_scylla(&req.question, &answer).await?;
    
    Ok(QueryResponse {
        answer,
        context,
    })
}
