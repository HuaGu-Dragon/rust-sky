use axum::{Json, Router, extract::State, routing::post};
use sky_pojo::dto::shopping_cart::CartDto;

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::UserId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/add", post(add))
}

async fn add(
    UserId(user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Json(cart): Json<CartDto>,
) -> ApiReturn<()> {
    server::shopping_cart::add(user_id, db, cart).await?;
    Ok(ApiResponse::success(()))
}
