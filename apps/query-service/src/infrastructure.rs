use shared_lib::error::Result;
use shared_lib::clients::init_qdrant;
use shared_lib::db::init_scylla;

pub async fn query_qdrant(_query: &str) -> Result<Vec<String>> {
    let _client = init_qdrant("http://qdrant:6334")?;
    Ok(vec!["Context 1".to_string(), "Context 2".to_string()])
}

pub async fn call_ollama(prompt: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://ollama:11434/api/generate")
        .json(&serde_json::json!({ "model": "llama3", "prompt": prompt, "stream": false }))
        .send()
        .await
        .map_err(|e| shared_lib::error::AppError::Internal(e.to_string()))?;
    
    let body: serde_json::Value = res.json().await
        .map_err(|e| shared_lib::error::AppError::Internal(e.to_string()))?;
    
    Ok(body["response"].as_str().unwrap_or("").to_string())
}

pub async fn save_to_scylla(question: &str, answer: &str) -> Result<()> {
    let session = init_scylla("scylla://scylla:9042").await?;
    let query = "INSERT INTO chat_history (id, question, answer, timestamp) VALUES (uuid(), ?, ?, now())";
    let prepared = session
        .prepare(query)
        .await
        .map_err(|e| shared_lib::error::AppError::Database(e.to_string()))?;
    session
        .execute_unpaged(&prepared, (question, answer))
        .await
        .map_err(|e| shared_lib::error::AppError::Database(e.to_string()))?;
    Ok(())
}
