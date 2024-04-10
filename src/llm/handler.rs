use super::{
    model::{DocReadingError, DocReadingSuccess},
    usecase,
};
use crate::startup::setting::Setting;
use axum::{http, response::IntoResponse, Json};
use std::sync::Arc;

pub async fn doc_reading(setting: Arc<Setting>) -> impl IntoResponse {
    let content = usecase::doc_reading(setting).await;

    match content {
        Ok(r) => (http::StatusCode::OK, Json(DocReadingSuccess { content: r })).into_response(),
        Err(e) => (
            http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(DocReadingError {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}
