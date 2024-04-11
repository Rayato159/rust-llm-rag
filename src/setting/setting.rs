use std::sync::Arc;

use config::Config;

#[derive(Debug, Clone)]
pub struct Setting {
    pub server: Server,
    pub vector_db: VectorDb,
    pub doc: Doc,
}

#[derive(Debug, Clone)]
pub struct VectorDb {
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub timeout: u32,
    pub max_payload: u64,
    pub max_buffer_size: usize,
}

#[derive(Debug, Clone)]
pub struct Doc {
    pub path: String,
}

impl Setting {
    pub fn new() -> Arc<Self> {
        let settings = Config::builder()
            .add_source(config::File::with_name("./Setting.toml"))
            .build()
            .unwrap();

        Arc::new(Setting {
            server: Server {
                port: settings.get_int("server.port").unwrap() as u16,
                timeout: settings.get_int("server.timeout").unwrap() as u32,
                max_payload: settings.get_int("server.max_payload").unwrap() as u64,
                max_buffer_size: settings.get_int("server.max_buffer_size").unwrap() as usize,
            },
            vector_db: VectorDb {
                host: settings.get_string("vector_db.host").unwrap(),
                port: settings.get_int("vector_db.port").unwrap() as u16,
            },
            doc: Doc {
                path: settings.get_string("doc.path").unwrap(),
            },
        })
    }
}
