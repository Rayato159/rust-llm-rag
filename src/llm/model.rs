use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DocReadingSuccess {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocReadingError {
    pub error: String,
}
