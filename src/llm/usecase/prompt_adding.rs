use super::errors;
use crate::{
    infrastructure::vector_db::QdrantDb,
    llm::model::{PromptAddingReq, PromptAddingSuccess},
};
use std::sync::Arc;

pub async fn prompt_adding(
    req: PromptAddingReq,
    db: Arc<QdrantDb>,
) -> Result<PromptAddingSuccess, errors::PromptAdding> {
    unimplemented!("Prompt adding is not implemented yet")
}
