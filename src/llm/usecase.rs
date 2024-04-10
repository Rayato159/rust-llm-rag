use super::errors::DocReading;
use crate::startup::setting::Setting;
use std::{fs, sync::Arc};

pub async fn doc_reading(setting: Arc<Setting>) -> Result<String, DocReading> {
    let content = fs::read_to_string(setting.doc.path.clone()).map_err(|_| DocReading)?;
    Ok(content)
}
