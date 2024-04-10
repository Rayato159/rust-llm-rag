use super::model::{DocReadingSuccess, Error, PromptAddingReq, PromptAddingSuccess};
use crate::llm::usecase;
use crate::{infrastructure::vector_db::QdrantDb, startup::setting::Setting};
use axum::{extract, http, response::IntoResponse, Json};
use std::sync::Arc;

pub async fn doc_reading(setting: Arc<Setting>) -> impl IntoResponse {
    let document = usecase::doc_reading::doc_reading(setting).await;

    match document {
        Ok(r) => (
            http::StatusCode::OK,
            Json(DocReadingSuccess { document: r }),
        )
            .into_response(),
        Err(e) => (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(Error {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub async fn prompt_adding(
    extract::Json(payload): extract::Json<PromptAddingReq>,
    db: Arc<QdrantDb>,
) -> impl IntoResponse {
    let result = usecase::prompt_adding::prompt_adding(payload, db).await;

    match result {
        Ok(r) => (
            http::StatusCode::OK,
            Json(PromptAddingSuccess { prompt: r.prompt }),
        )
            .into_response(),
        Err(e) => (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(Error {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}
