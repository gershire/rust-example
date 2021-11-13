use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct Grpc {
    pub socket_address: String,
}

#[derive(Debug, Deserialize)]
pub struct Kafka {
    pub bootstrap_server: String,
    pub topic: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub grpc: Grpc,
    pub kafka: Kafka,
}

impl Settings {
    pub fn new(path: String) -> Result<Self, ConfigError> {
        let mut s = Config::default();
        s.merge(File::with_name(&*path))?;
        s.try_into()
    }
}