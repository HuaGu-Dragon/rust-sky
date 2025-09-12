use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, put},
};
use sky_pojo::{
    dto::dish::{DishDto, DishQueryDelete, DishQueryDto},
    vo::{
        Page,
        dish::{DishDetailVO, DishVO},
    },
};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::Id, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", put(update).delete(delete_dish).post(save))
        .route("/{id}", get(get_dish))
        .route("/page", get(page))
}

async fn save(
    Id(id): Id,
    State(AppState { db }): State<AppState>,
    Json(category): Json<DishDto>,
) -> ApiReturn<()> {
    server::dish::save(id, db, category).await?;
    Ok(ApiResponse::success(()))
}

async fn update(
    Id(id): Id,
    State(AppState { db }): State<AppState>,
    Json(category): Json<DishDto>,
) -> ApiReturn<()> {
    server::dish::update(id, db, category).await?;
    Ok(ApiResponse::success(()))
}

async fn get_dish(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
    Path(id): Path<i64>,
) -> ApiReturn<DishDetailVO> {
    let dish = server::dish::get(db, id).await?;
    Ok(ApiResponse::success(dish))
}

async fn delete_dish(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
    Query(query): Query<DishQueryDelete>,
) -> ApiReturn<()> {
    let ids = query
        .ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    server::dish::delete(db, ids).await?;
    Ok(ApiResponse::success(()))
}

async fn page(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
    Query(query): Query<DishQueryDto>,
) -> ApiReturn<Page<DishVO>> {
    let dishes = server::dish::page(db, query).await?;
    Ok(ApiResponse::success(dishes))
}
