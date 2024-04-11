use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptAddingReq {
    pub prompt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptAddingSuccess {
    pub embedded: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error: String,
}
