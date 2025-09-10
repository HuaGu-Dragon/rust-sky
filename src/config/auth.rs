use jsonwebtoken::Algorithm;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthConfig {
    secret: Option<String>,
    algorithm: Option<Algorithm>,
    expiration: Option<u64>,
}

impl AuthConfig {
    pub fn secret(&self) -> &str {
        self.secret.as_deref().unwrap_or("default_secret")
    }

    pub fn algorithm(&self) -> Algorithm {
        self.algorithm.unwrap_or_default()
    }

    pub fn expiration(&self) -> u64 {
        self.expiration.unwrap_or(3600)
    }
}
