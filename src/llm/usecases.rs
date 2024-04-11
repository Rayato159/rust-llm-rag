use super::errors;
use crate::{
    infrastructure::vector_db::QdrantDb,
    llm::model::{PromptAddingReq, PromptAddingSuccess},
};
use async_trait::async_trait;
use ollama_rs::Ollama;
use std::sync::Arc;

const EMBEDDINGS_MODEL: &str = "nomic-embed-text:latest";

#[async_trait]
pub trait Usecases {
    async fn prompt_adding(
        &self,
        req: PromptAddingReq,
    ) -> Result<PromptAddingSuccess, errors::PromptAdding>;
}

#[derive(Clone)]
pub struct UsecasesImpl {
    db: Arc<QdrantDb>,
}

impl UsecasesImpl {
    pub fn new(db: Arc<QdrantDb>) -> Arc<Self> {
        Arc::new(Self { db })
    }
}

#[async_trait]
impl Usecases for UsecasesImpl {
    async fn prompt_adding(
        &self,
        req: PromptAddingReq,
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
}
