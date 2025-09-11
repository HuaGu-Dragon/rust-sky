use axum::Router;

use crate::app::AppState;

mod category;
mod employee;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/employee", employee::create_router())
        .nest("/category", category::create_router())
}
