use std::sync::Arc;

use config::Config;

#[derive(Debug, Clone)]
pub struct Setting {
    pub qdrant_host: String,
    pub qdrant_port: u16,
}

impl Setting {
    pub fn new() -> Arc<Self> {
        let settings = Config::builder()
            .add_source(config::File::with_name("./Setting.toml"))
            .build()
            .unwrap();

        Arc::new(Setting {
            qdrant_host: settings.get_string("vector_db.qdrant_host").unwrap(),
            qdrant_port: settings.get_int("vector_db.qdrant_port").unwrap() as u16,
        })
    }
}
