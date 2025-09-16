use axum::{
    Router,
    extract::{Path, State},
    routing::get,
};
use axum_extra::extract::Query;
use sky_pojo::{
    dto::setmeal::SetmealQueryId,
    vo::setmeal::{SetmealVo, UserSetmealDishVo},
};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::UserId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/list", get(list))
        .route("/dish/{id}", get(get_dish))
}

async fn list(
    UserId(_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Query(SetmealQueryId { category_id }): Query<SetmealQueryId>,
) -> ApiReturn<Vec<SetmealVo>> {
    let dishes = server::setmeal::list(db, category_id).await?;
    Ok(ApiResponse::success(dishes))
}

async fn get_dish(
    UserId(_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> ApiReturn<Vec<UserSetmealDishVo>> {
    let dish = server::setmeal::get_dish(db, id).await?;
    Ok(ApiResponse::success(dish))
}
