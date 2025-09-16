use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WxConfig {
    app_id: String,
    app_secret: String,
}

impl WxConfig {
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    pub fn app_secret(&self) -> &str {
        &self.app_secret
    }
}
