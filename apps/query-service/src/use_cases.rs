use shared_lib::domain::{QueryRequest, QueryResponse};
use shared_lib::error::{AppError, Result};
use crate::infrastructure::{query_qdrant, call_ollama, save_to_scylla};

pub async fn handle_query(req: QueryRequest) -> Result<QueryResponse> {
    let context = query_qdrant(&req.question).await?;
    let prompt = format!(
        "Context: {}\nHistory: {}\nQuestion: {}",
        context.join("\n"),
        req.chat_history.join("\n"),
        req.question
    );
    let answer = match std::env::var("LLM_PROVIDER").unwrap_or_else(|_| "ollama".to_string()).as_str() {
        "vllm" => call_vllm(&prompt).await?,
        _ => call_ollama(&prompt).await?,
    };
    save_to_scylla(&req.question, &answer).await?;
    
    Ok(QueryResponse {
        answer,
        context,
    })
}

async fn call_vllm(prompt: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let endpoint = std::env::var("LLM_ENDPOINT").unwrap_or_else(|_| "http://vllm:8000/v1/chat/completions".to_string());
    let model = std::env::var("LLM_MODEL").unwrap_or_else(|_| "meta-llama/Llama-3.1-8B-Instruct".to_string());

    let res = client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7,
            "max_tokens": 512
        }))
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("vLLM request failed: {}", e)))?;

    let body: serde_json::Value = res
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("vLLM response parse failed: {}", e)))?;

    let answer = body["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| AppError::Internal("vLLM response missing content".to_string()))?
        .to_string();

    Ok(answer)
}
