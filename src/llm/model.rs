use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocAddingReq {
    pub doc: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocAddingSuccess {
    pub id: String,
    pub embedded: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error: String,
}
