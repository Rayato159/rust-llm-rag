use super::errors;
use crate::infrastructure::vector_db::QdrantDb;
use async_trait::async_trait;
use ollama_rs::Ollama;
use qdrant_client::qdrant::PointStruct;
use serde_json::json;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

const EMBEDDINGS_MODEL: &str = "nomic-embed-text:latest";
const COLLECTION: &str = "docs";

#[async_trait]
pub trait Usecases {
    async fn doc_adding(&self, doc: String) -> Result<String, errors::Error>;
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

    async fn embedding(&self, doc: String) -> Result<Vec<f32>, errors::Error> {
        let doc_embedded = &self
            .ollama
            .generate_embeddings(EMBEDDINGS_MODEL.to_string(), doc.clone(), None)
            .await
            .map_err(|e| {
                errors::Error::new(&format!("Error generating embeddings: {}", e.to_string()))
            })?;

        let result: Vec<f32> = doc_embedded.embeddings.iter().map(|&x| x as f32).collect();

        Ok(result)
    }
}

#[async_trait]
impl Usecases for UsecasesImpl {
    async fn doc_adding(&self, doc: String) -> Result<String, errors::Error> {
        let doc_embedded = &self.embedding(doc.clone()).await?;

        let payload = json!({
            "doc": doc.clone(),
        })
        .try_into()
        .map_err(|_| {
            errors::Error::new(&format!(
                "Error converting payload to json: {}",
                doc.clone()
            ))
        })?;

        let id = Uuid::new_v4().to_string();
        let points = vec![PointStruct::new(id.clone(), doc_embedded.clone(), payload)];

        let operation_info = self
            .db
            .client
            .upsert_points_blocking(COLLECTION.to_string(), None, points, None)
            .await
            .map_err(|e| {
                errors::Error::new(&format!("Error upserting points: {}", e.to_string()))
            })?;

        info!("{:?}", operation_info);

        Ok(format!("Document added with id: {}", id))
    }
}
