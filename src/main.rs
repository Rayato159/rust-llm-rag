use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::http::{Method, StatusCode};
use axum::routing::get;
use axum::{BoxError, Router};
use rust_llm_rag::infrastructure::vector_db::{init_client, QdrantDb};
use rust_llm_rag::startup::setting::Setting;
use tower::timeout::TimeoutLayer;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let setting = Setting::new();

    let vector_db_client = init_client(setting.clone());
    let qdrant_db = QdrantDb::new(vector_db_client);

    let app = Router::new()
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_origin(Any),
        )
        .layer(RequestBodyLimitLayer::new(
            setting.server.body_limit.try_into().unwrap(),
        ))
        .route("/", get(|| async { "Hello, World!" }))
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: BoxError| async {
                    StatusCode::REQUEST_TIMEOUT
                }))
                .layer(TimeoutLayer::new(Duration::from_secs(
                    setting.server.timeout.try_into().unwrap(),
                ))),
        )
        .fallback(|| async { "Not Found" });

    let uri = format!("0.0.0.0:{}", setting.server.port);
    let listener = tokio::net::TcpListener::bind(&uri).await.unwrap();

    info!("🦀 Server is starting on: :{}", setting.server.port);

    axum::serve(listener, app).await.unwrap();
}
