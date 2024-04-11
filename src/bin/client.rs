use axum::Router;
use rust_llm_rag::infrastructure::vector_db::{init_client, QdrantDb};
use rust_llm_rag::llm::{handlers, usecases};
use rust_llm_rag::setting::setting::Setting;
use socketioxide::extract::Data;
use socketioxide::{extract::SocketRef, SocketIo};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let setting = Setting::new();

    let vector_db_client = init_client(Arc::clone(&setting));
    let qdrant_db = QdrantDb::new(vector_db_client);

    let llm_usecases = usecases::UsecasesImpl::new(Arc::clone(&qdrant_db));
    let llm_handlers = handlers::Handlers::new(Arc::clone(&llm_usecases));

    let (socket_layer, io) = SocketIo::builder()
        .max_payload(Arc::clone(&setting).server.max_payload)
        .max_buffer_size(Arc::clone(&setting).server.max_buffer_size)
        .build_layer();

    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on(
            "prompt",
            |s: SocketRef, Data::<String>(prompt)| async move {
                let result = llm_handlers.chatting(prompt.clone()).await;

                s.emit("result", result).ok();
            },
        );
    });

    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .layer(socket_layer);

    let uri = format!("0.0.0.0:{}", setting.server.port);
    let listener = tokio::net::TcpListener::bind(&uri).await.unwrap();

    info!("🦀 Server is starting on: :{}", setting.server.port);

    axum::serve(listener, app).await.unwrap();
}
