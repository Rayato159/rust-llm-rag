use super::model::{Error, PromptAddingReq, PromptAddingSuccess};
use super::usecases;
use crate::infrastructure::vector_db::QdrantDb;
use axum::{extract, http, response::IntoResponse, Json};
use std::sync::Arc;

pub async fn prompt_adding(
    extract::Json(req): extract::Json<PromptAddingReq>,
    db: Arc<QdrantDb>,
) -> impl IntoResponse {
    let result = usecases::prompt_adding(req, db).await;

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
