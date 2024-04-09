use std::sync::Arc;

use config::Config;

#[derive(Debug, Clone)]
pub struct Setting {
    pub server: Server,
    pub vector_db: VectorDb,
}

#[derive(Debug, Clone)]
pub struct VectorDb {
    pub host: String,
    pub port: u16,
}
#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u32,
    pub timeout: u32,
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
                body_limit: settings.get_int("server.body_limit").unwrap() as u32,
                timeout: settings.get_int("server.timeout").unwrap() as u32,
            },
            vector_db: VectorDb {
                host: settings.get_string("vector_db.host").unwrap(),
                port: settings.get_int("vector_db.port").unwrap() as u16,
            },
        })
    }
}
