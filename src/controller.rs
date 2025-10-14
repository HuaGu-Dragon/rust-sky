use axum::Router;

use crate::app::AppState;

mod admin;
mod upload;
mod user;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/admin", admin::create_router())
        .nest("/user", user::create_router())
        .nest("/upload", upload::create_router())
}
