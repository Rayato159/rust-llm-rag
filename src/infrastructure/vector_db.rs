use crate::setting::setting::Setting;
use qdrant_client::client::QdrantClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct QdrantDb {
    pub client: Arc<QdrantClient>,
}

pub fn init_client(setting: Arc<Setting>) -> QdrantClient {
    let host = setting.vector_db.host.clone();
    let port = setting.vector_db.port.clone();

    let uri = format!("http://{}:{}", host, port);

    QdrantClient::from_url(&uri).build().unwrap()
}

impl QdrantDb {
    pub fn new(client: QdrantClient) -> Arc<Self> {
        Arc::new(QdrantDb {
            client: Arc::new(client),
        })
    }
}
