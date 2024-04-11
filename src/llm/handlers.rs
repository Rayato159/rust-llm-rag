use super::usecases::Usecases;
use std::sync::Arc;
use tracing::{error, info};

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

    pub async fn doc_adding(&self, doc: String) {
        let result = &self.usecases.doc_adding(doc).await;

        match result {
            Ok(_) => {
                info!("Document added successfully");
            }
            Err(e) => {
                error!("Error adding the document: {:?}", e);
            }
        }
    }
}
