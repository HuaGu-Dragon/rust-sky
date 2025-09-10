use axum::Router;

use crate::app::AppState;

mod admin;

pub fn create_router() -> Router<AppState> {
    Router::new().nest("/admin", admin::create_router())
}
