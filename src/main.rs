use rust_llm_rag::infrastructure::vector_db::{init_client, QdrantDb};
use rust_llm_rag::startup::setting::Setting;

fn main() {
    let setting = Setting::new();

    let vector_db_client = init_client(setting.clone());
    let qdrant_db = QdrantDb::new(vector_db_client);

    println!("{:?}", setting);
}
