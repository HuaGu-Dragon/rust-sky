use axum::Router;

use crate::app::AppState;

mod category;
mod dish;
mod login;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/user", login::create_router())
        .nest("/category", category::create_router())
        .nest("/dish", dish::create_router())
}
