use super::usecases::Usecases;
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

    pub async fn chatting(&self, prompt: String) -> String {
        let result = &self.usecases.doc_adding(prompt).await;

        match result {
            Ok(r) => r.to_string(),
            Err(e) => format!("Error adding the document: {:?}", e),
        }
    }
}
