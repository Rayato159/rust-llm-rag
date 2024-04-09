use crate::startup::setting::Setting;
use qdrant_client::client::QdrantClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct QdrantDb {
    pub client: Arc<QdrantClient>,
}

pub fn init_client(setting: Arc<Setting>) -> QdrantClient {
    let host = setting.qdrant_host.clone();
    let port = setting.qdrant_port.clone();

    let uri = format!("http://{}:{}", host, port);

    QdrantClient::from_url(&uri).build().unwrap()
}

impl QdrantDb {
    pub fn new(client: QdrantClient) -> Self {
        QdrantDb {
            client: Arc::new(client),
        }
    }
}
