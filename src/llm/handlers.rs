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

    pub async fn chatting(&self, prompt: String, model: String) -> String {
        let history_result = &self.usecases.doc_adding(prompt.clone()).await;

        let history_prompt: String;
        match history_result {
            Ok(r) => history_prompt = r.to_string(),
            Err(e) => return format!("Error adding the document: {:?}", e),
        };

        let result = &self.usecases.chatting(prompt, history_prompt, model).await;

        result.to_string()
    }
}
