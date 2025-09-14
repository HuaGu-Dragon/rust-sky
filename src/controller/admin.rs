use axum::Router;

use crate::app::AppState;

mod category;
mod common;
mod dish;
mod employee;
mod setmeal;
mod shop;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/employee", employee::create_router())
        .nest("/category", category::create_router())
        .nest("/common", common::create_router())
        .nest("/dish", dish::create_router())
        .nest("/setmeal", setmeal::create_router())
        .nest("/shop", shop::create_router())
}
