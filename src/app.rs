use axum::Router;
use sea_orm::DatabaseConnection;

use crate::{config, database, logger, server::Server};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) {
    logger::init();

    let db = database::init().await;
    let state = AppState::new(db);

    let server = Server::new(config::get().server());
    server.start(router, state).await;
}
