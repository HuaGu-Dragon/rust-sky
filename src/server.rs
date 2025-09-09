use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::{app::AppState, config::server::ServerConfig};

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    // TODO: Add TLS support
    // TODO: Add graceful shutdown
    // TODO: use anyhow or thiserror for better error handling
    pub async fn start(&self, router: Router<AppState>, state: AppState) {
        let router = self.build_router(router, state);
        let port = self.config.port();

        let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
            .await
            .expect("Failed to bind to address");
        info!("initialized with port(s): {port} (http)");

        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .expect("Failed to start the server");
    }

    pub fn build_router(&self, router: Router<AppState>, state: AppState) -> Router {
        Router::new().merge(router).with_state(state)
    }
}
