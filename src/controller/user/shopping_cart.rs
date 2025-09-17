use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use sky_pojo::{dto::shopping_cart::CartDto, vo::shopping_cart::CartVO};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::UserId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/add", post(add))
        .route("/list", get(list))
}

async fn add(
    UserId(user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Json(cart): Json<CartDto>,
) -> ApiReturn<()> {
    server::shopping_cart::add(user_id, db, cart).await?;
    Ok(ApiResponse::success(()))
}

async fn list(
    UserId(user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
) -> ApiReturn<Vec<CartVO>> {
    let carts = server::shopping_cart::list(user_id, db).await?;
    Ok(ApiResponse::success(carts))
}
