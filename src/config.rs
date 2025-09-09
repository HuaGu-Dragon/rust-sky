use std::sync::LazyLock;

use serde::Deserialize;

use crate::config::{database::DatabaseConfig, server::ServerConfig};

mod database;
pub mod server;

static CONFIG: LazyLock<AppConfig> = LazyLock::new(AppConfig::load);
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    database: DatabaseConfig,
    server: ServerConfig,
}

impl AppConfig {
    // TODO: Use anyhow or thiserror for better error handling
    pub fn load() -> Self {
        config::Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(config::FileFormat::Toml)
                    .required(true),
            )
            .build()
            .expect("Failed to build config")
            .try_deserialize()
            .expect("Failed to deserialize config")
    }

    pub fn database(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn server(&self) -> &server::ServerConfig {
        &self.server
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
