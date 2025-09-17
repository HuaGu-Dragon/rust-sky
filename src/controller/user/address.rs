use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use sky_pojo::{
    dto::{StateQuery, address::AddressDto},
    vo::address::AddressVO,
};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::UserId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(add).put(update_address))
        .route("/{id}", get(query_address).delete(remove))
        .route("/list", get(list))
        .route("/default", get(default).put(set_default))
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

async fn default(
    UserId(user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
) -> ApiReturn<Option<AddressVO>> {
    let address = server::address::default_address(user_id, db).await?;
    Ok(ApiResponse::success(address))
}

async fn update_address(
    UserId(_user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Json(address): Json<AddressDto>,
) -> ApiReturn<()> {
    server::address::update(address, db).await?;
    Ok(ApiResponse::success(()))
}

async fn remove(
    UserId(_user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Query(StateQuery { id }): Query<StateQuery>,
) -> ApiReturn<()> {
    server::address::remove(id, db).await?;
    Ok(ApiResponse::success(()))
}

async fn query_address(
    UserId(_user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Query(StateQuery { id }): Query<StateQuery>,
) -> ApiReturn<AddressVO> {
    let address = server::address::get(id, db).await?;
    Ok(ApiResponse::success(address))
}

async fn set_default(
    UserId(_user_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Json(StateQuery { id }): Json<StateQuery>,
) -> ApiReturn<()> {
    server::address::set_default(id, db).await?;
    Ok(ApiResponse::success(()))
}
