use axum::Router;

use crate::app::AppState;

mod category;
mod dish;
mod login;
mod setmeal;
mod shopping_cart;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/user", login::create_router())
        .nest("/category", category::create_router())
        .nest("/dish", dish::create_router())
        .nest("/setmeal", setmeal::create_router())
        .nest("/shoppingCart", shopping_cart::create_router())
}
