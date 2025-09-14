use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl RedisConfig {
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap_or(6379)
    }
}
