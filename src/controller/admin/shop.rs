use axum::{
    Router,
    extract::{Path, State},
    routing::{get, put},
};
use redis::AsyncTypedCommands;

use crate::{
    app::AppState,
    server::{ApiReturn, error::ApiError, response::ApiResponse},
};

const KEY: &str = "SHOP_STATUS";

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/{status}", put(set_status))
        .route("/status", get(get_status))
}

async fn set_status(
    State(AppState { redis, .. }): State<AppState>,
    Path(status): Path<i32>,
) -> ApiReturn<()> {
    let mut conn = redis.clone();
    conn.set(KEY, status)
        .await
        .map_err(|_| ApiError::Internal)?;

    Ok(ApiResponse::success(()))
}

async fn get_status(State(AppState { redis, .. }): State<AppState>) -> ApiReturn<i32> {
    let mut conn = redis.clone();
    let status = conn
        .get_int(KEY)
        .await
        .map_err(|_| ApiError::Internal)?
        .unwrap_or(0);

    Ok(ApiResponse::success(status as i32))
}
