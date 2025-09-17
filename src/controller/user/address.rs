use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use sky_pojo::{dto::address::AddressDto, vo::address::AddressVO};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::UserId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(add))
        .route("/list", get(list))
}

async fn add(
    UserId(user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Json(address): Json<AddressDto>,
) -> ApiReturn<()> {
    server::address::add(user_id, db, address).await?;
    Ok(ApiResponse::success(()))
}

async fn list(
    UserId(user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
) -> ApiReturn<Vec<AddressVO>> {
    let addresses = server::address::list(user_id, db).await?;
    Ok(ApiResponse::success(addresses))
}
