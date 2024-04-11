use super::errors;
use crate::{
    infrastructure::vector_db::QdrantDb,
    llm::model::{PromptAddingReq, PromptAddingSuccess},
};
use async_trait::async_trait;
use ollama_rs::Ollama;
use tracing::info;
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
    ollama: Ollama,
    db: Arc<QdrantDb>,
}

impl UsecasesImpl {
    pub fn new(db: Arc<QdrantDb>) -> Arc<Self> {
        Arc::new(Self {
            db,
            ollama: Ollama::default(),
        })
    }
}

#[async_trait]
impl Usecases for UsecasesImpl {
    async fn prompt_adding(
        &self,
        req: PromptAddingReq,
    ) -> Result<PromptAddingSuccess, errors::PromptAdding> {
        let prompt_embedded = &self
            .ollama
            .generate_embeddings(EMBEDDINGS_MODEL.to_string(), req.clone().prompt, None)
            .await
            .map_err(|e| {
                println!("{:?}", e);
                errors::PromptAdding
            })?;

        

        info!("embeddeding prompt completed");

        Ok(PromptAddingSuccess {
            embedded: prompt_embedded.clone().embeddings,
        })
    }
}
