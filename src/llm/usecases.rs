use super::errors;
use crate::{
    infrastructure::vector_db::QdrantDb,
    llm::model::{DocAddingReq, DocAddingSuccess},
};
use async_trait::async_trait;
use ollama_rs::Ollama;
use qdrant_client::qdrant::PointStruct;
use serde_json::json;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{error, info};

const EMBEDDINGS_MODEL: &str = "nomic-embed-text:latest";
const COLLECTION: &str = "docs";

#[async_trait]
pub trait Usecases {
    async fn doc_adding(
        &self,
        req: DocAddingReq,
    ) -> Result<DocAddingSuccess, errors::DocAdding>;
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
    async fn doc_adding(
        &self,
        req: DocAddingReq,
    ) -> Result<DocAddingSuccess, errors::DocAdding> {
        let doc_embedded = &self
            .ollama
            .generate_embeddings(EMBEDDINGS_MODEL.to_string(), req.clone().doc, None)
            .await
            .map_err(|e| {
                error!("{:?}", e);
                errors::DocAdding
            })?;

        let doc_embedded_vec: Vec<f32> = doc_embedded
            .embeddings
            .iter()
            .map(|&x| x as f32)
            .collect();

        let payload = json!({
            "doc": req.doc,
        })
        .try_into()
        .map_err(|e| {
            error!("{:?}", e);
            errors::DocAdding
        })?;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let points = vec![PointStruct::new(now, doc_embedded_vec, payload)];

        self.db
            .client
            .upsert_points(COLLECTION, None, points, None)
            .await
            .map_err(|e| {
                error!("{:?}", e);
                errors::DocAdding
            })?;

        info!("embeddeding doc completed");

        Ok(DocAddingSuccess {
            embedded: doc_embedded.clone().embeddings,
        })
    }
}
