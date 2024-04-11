use super::errors;
use crate::{
    infrastructure::vector_db::QdrantDb,
    llm::model::{PromptAddingReq, PromptAddingSuccess},
};
use ollama_rs::Ollama;
use std::sync::Arc;

const EMBEDDINGS_MODEL: &str = "nomic-embed-text:latest";

pub async fn prompt_adding(
    req: PromptAddingReq,
    db: Arc<QdrantDb>,
) -> Result<PromptAddingSuccess, errors::PromptAdding> {
    let ollama = Ollama::default();

    let prompt_embedded = ollama
        .generate_embeddings(EMBEDDINGS_MODEL.to_string(), req.clone().prompt, None)
        .await
        .map_err(|e| {
            println!("{:?}", e);
            errors::PromptAdding
        })?;

    dbg!("{:?}", prompt_embedded);

    Ok(PromptAddingSuccess {
        prompt: req.clone().prompt,
    })
}
