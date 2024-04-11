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
use tracing::{error, info};
use uuid::Uuid;

const EMBEDDINGS_MODEL: &str = "nomic-embed-text:latest";
const COLLECTION: &str = "docs";

#[async_trait]
pub trait Usecases {
    async fn doc_adding(&self, req: DocAddingReq) -> Result<DocAddingSuccess, errors::DocAdding>;
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

    async fn embedding(&self, doc: String) -> Result<Vec<f32>, errors::DocEmbedding> {
        let doc_embedded = &self
            .ollama
            .generate_embeddings(EMBEDDINGS_MODEL.to_string(), doc.clone(), None)
            .await
            .map_err(|e| {
                error!("{:?}", e);
                errors::DocEmbedding
            })?;

        let result: Vec<f32> = doc_embedded.embeddings.iter().map(|&x| x as f32).collect();

        Ok(result)
    }
}

#[async_trait]
impl Usecases for UsecasesImpl {
    async fn doc_adding(&self, req: DocAddingReq) -> Result<DocAddingSuccess, errors::DocAdding> {
        let doc = req.clone().doc;
        let doc_embedded = &self.embedding(doc).await.map_err(|e| {
            error!("{:?}", e);
            errors::DocAdding
        })?;

        let payload = json!({
            "doc": req.doc,
        })
        .try_into()
        .map_err(|e| {
            error!("{:?}", e);
            errors::DocAdding
        })?;

        let id = Uuid::new_v4().to_string();
        let points = vec![PointStruct::new(id.clone(), doc_embedded.clone(), payload)];

        let operation_info = self
            .db
            .client
            .upsert_points_blocking(COLLECTION.to_string(), None, points, None)
            .await
            .map_err(|e| {
                error!("{:?}", e);
                errors::DocAdding
            })?;

        info!("{:?}", operation_info);

        Ok(DocAddingSuccess {
            id: id.clone(),
            embedded: doc_embedded.clone(),
        })
    }
}
