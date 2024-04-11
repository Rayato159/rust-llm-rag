use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptAddingReq {
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptAddingSuccess {
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error: String,
}
