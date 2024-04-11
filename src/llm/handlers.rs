use super::model::{DocAddingReq, Error};
use super::usecases::Usecases;
use axum::{extract, http, response::IntoResponse, Json};
use std::sync::Arc;

pub struct Handlers<T>
where
    T: Usecases + Clone + Send + Sync + 'static,
{
    usecases: Arc<T>,
}

impl<T> Handlers<T>
where
    T: Usecases + Clone + Send + Sync + 'static,
{
    pub fn new(usecases: Arc<T>) -> Arc<Self> {
        Arc::new(Self {
            usecases: Arc::clone(&usecases),
        })
    }

    pub async fn doc_adding(
        &self,
        extract::Json(req): extract::Json<DocAddingReq>,
    ) -> impl IntoResponse {
        let result = &self.usecases.doc_adding(req).await;

        match result {
            Ok(r) => (http::StatusCode::OK, Json(r)).into_response(),
            Err(e) => (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(Error {
                    error: e.to_string(),
                }),
            )
                .into_response(),
        }
    }
}
