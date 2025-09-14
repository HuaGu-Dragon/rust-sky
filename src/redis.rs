use std::time::Duration;

use redis::{
    Client,
    aio::{ConnectionManager, ConnectionManagerConfig},
};

use crate::config;

pub async fn init() -> ConnectionManager {
    let config = config::get().redis();
    let client = Client::open(format!("redis://{}:{}/", config.host(), config.port())).unwrap();
    let manger_config =
        ConnectionManagerConfig::new().set_connection_timeout(Duration::from_secs(10));

    ConnectionManager::new_with_config(client, manger_config)
        .await
        .expect("Failed to connect to Redis")
}
