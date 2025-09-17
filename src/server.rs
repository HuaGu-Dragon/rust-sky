use std::{net::SocketAddr, time::Duration};

use axum::{
    Router,
    extract::{ConnectInfo, Request},
};
use tokio::net::TcpListener;
use tower_http::{
    auth::AsyncRequireAuthorizationLayer, cors::CorsLayer, limit::RequestBodyLimitLayer,
    normalize_path::NormalizePathLayer, timeout::TimeoutLayer, trace::TraceLayer,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    app::AppState,
    config::server::ServerConfig,
    server::{
        auth::JwtAuthKey, error::ApiResult, latency::LatencyLayer, middleware::AuthLayer,
        response::ApiResponse,
    },
};

pub mod address;
pub mod auth;
pub mod category;
pub mod dish;
pub mod employee;
pub mod error;
pub mod extract;
mod latency;
pub mod middleware;
pub mod response;
pub mod setmeal;
pub mod shopping_cart;
pub mod user;

pub type ApiReturn<T> = ApiResult<ApiResponse<T>>;

const ENABLE: i32 = 1;
const DISABLE: i32 = 0;

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
        Router::new()
            .merge(router)
            .layer(NormalizePathLayer::trim_trailing_slash())
            .layer(RequestBodyLimitLayer::new(1024 * 1024 * 10))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request| {
                        let id = Uuid::new_v4();
                        let span = tracing::info_span!(
                            "http_request",
                            id = %id,
                            user_type = tracing::field::Empty,
                            user_id = tracing::field::Empty,
                            error = tracing::field::Empty,
                            method = %request.method(),
                            uri = %request.uri(),
                            version = ?request.version(),
                            addr = tracing::field::Empty
                        );

                        if let Some(user) = request.extensions().get::<(JwtAuthKey, i64)>() {
                            span.record("user_type", format!("{:?}", user.0));
                            span.record("user_id", user.1);
                        }
                        if let Some(error) = request.extensions().get::<String>() {
                            span.record("error", error);
                        }

                        span
                    })
                    .on_request(|request: &Request, span: &tracing::Span| {
                        if let Some(ConnectInfo(addr)) =
                            request.extensions().get::<ConnectInfo<SocketAddr>>()
                        {
                            span.record("addr", addr.to_string());
                        }
                    })
                    .on_failure(())
                    .on_response(LatencyLayer),
            )
            .layer(AsyncRequireAuthorizationLayer::new(AuthLayer))
            .layer(CorsLayer::permissive())
            .layer(TimeoutLayer::new(Duration::from_secs(30))) // TODO: make configurable
            .with_state(state)
    }
}

#[macro_export]
macro_rules! update_params {
    ($active_model:expr, $field:ident, $value:expr) => {
        $active_model.$field = ActiveValue::Set($value);
    };
}
