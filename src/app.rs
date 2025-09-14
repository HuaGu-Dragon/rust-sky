use ::redis::aio::ConnectionManager;
use axum::Router;
use sea_orm::DatabaseConnection;

use crate::{config, database, logger, redis, server::Server};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: ConnectionManager,
}

impl AppState {
    pub fn new(db: DatabaseConnection, redis: ConnectionManager) -> Self {
        Self { db, redis }
    }
}

pub async fn run(router: Router<AppState>) {
    logger::init();

    let db = database::init().await;
    let redis = redis::init().await;
    let state = AppState::new(db, redis);

    let server = Server::new(config::get().server());
    server.start(router, state).await;
}
