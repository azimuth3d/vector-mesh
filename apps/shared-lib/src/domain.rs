use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct QueryRequest {
    pub question: String,
    pub chat_history: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct QueryResponse {
    pub answer: String,
    pub context: Vec<String>,
}
