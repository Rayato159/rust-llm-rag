use super::errors;
use crate::infrastructure::vector_db::QdrantDb;
use async_trait::async_trait;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use qdrant_client::qdrant::PointStruct;
use qdrant_client::qdrant::SearchPoints;
use serde_json::json;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

const EMBEDDINGS_MODEL: &str = "nomic-embed-text:latest";
const COLLECTION: &str = "docs";

#[async_trait]
pub trait Usecases {
    async fn doc_adding(&self, prompt: String) -> Result<String, errors::Error>;
    async fn chatting(&self, prompt: String, context: String, model: String) -> String;
}

#[derive(Clone)]
pub struct UsecasesImpl {
    db: Arc<QdrantDb>,
    ollama: Arc<Ollama>,
}

impl UsecasesImpl {
    pub fn new(db: Arc<QdrantDb>, ollama: Arc<Ollama>) -> Arc<Self> {
        Arc::new(Self { db, ollama })
    }

    async fn embedding(&self, prompt: String) -> Result<Vec<f32>, errors::Error> {
        let doc_embedded = &self
            .ollama
            .generate_embeddings(EMBEDDINGS_MODEL.to_string(), prompt.clone(), None)
            .await
            .map_err(|e| {
                errors::Error::new(&format!("Error generating embeddings: {}", e.to_string()))
            })?;

        let result: Vec<f32> = doc_embedded.embeddings.iter().map(|&x| x as f32).collect();

        Ok(result)
    }

    async fn doc_searching(&self, doc_embedded: &Vec<f32>) -> Result<String, errors::Error> {
        let result = &self
            .db
            .client
            .search_points(&SearchPoints {
                collection_name: COLLECTION.to_string(),
                vector: doc_embedded.to_vec(),

                limit: 3,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await
            .map_err(|e| {
                errors::Error::new(&format!("Error searching points: {}", e.to_string()))
            })?;

        if result.result.is_empty() {
            return Ok("".to_string());
        }

        let best_result = result
            .result
            .iter()
            .map(|r| {
                r.payload
                    .get("doc")
                    .and_then(|doc| doc.as_str())
                    .map(|doc| doc.to_string())
                    .unwrap_or_default()
            })
            .collect::<Vec<String>>()
            .join("\n");

        Ok(best_result.to_string())
    }
}

#[async_trait]
impl Usecases for UsecasesImpl {
    async fn doc_adding(&self, prompt: String) -> Result<String, errors::Error> {
        let doc_embedded = &self.embedding(prompt.clone()).await?;

        let payload = json!({
            "doc": prompt.clone(),
        })
        .try_into()
        .map_err(|_| {
            errors::Error::new(&format!(
                "Error converting payload to json: {}",
                prompt.clone()
            ))
        })?;

        let id = Uuid::new_v4().to_string();
        let points = vec![PointStruct::new(id.clone(), doc_embedded.clone(), payload)];

        let result = self.doc_searching(&doc_embedded).await?;

        let operation_info = self
            .db
            .client
            .upsert_points_blocking(COLLECTION.to_string(), None, points, None)
            .await
            .map_err(|e| {
                errors::Error::new(&format!("Error upserting points: {}", e.to_string()))
            })?;

        info!("Operation info: {:?}", operation_info);

        Ok(result)
    }

    async fn chatting(&self, prompt: String, context: String, model: String) -> String {
        let metaprompt = format!(
            "User's question:\n{}\n

            Relevant history:\n{}\n
        
            Please provide a response to the user's question, considering the relevant history.\n
            But don't tell user's that this anwser is from historical data.\n
            Just talk like a real human\n
            Answer:",
            prompt, context
        );

        let res = &self
            .ollama
            .generate(GenerationRequest::new(model, metaprompt))
            .await;

        match res {
            Ok(r) => r.response.clone(),
            Err(e) => format!("Error adding the document: {:?}", e),
        }
    }
}
