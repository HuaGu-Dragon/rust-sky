use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use sky_pojo::{
    dto::{
        QueryDelete, StateQuery,
        setmeal::{SetmealDto, SetmealPageQuery},
    },
    vo::{
        Page,
        setmeal::{SetmealDetailVo, SetmealVo},
    },
};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::AdminId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(save).delete(delete_meal).put(update))
        .route("/{id}", get(get_meal))
        .route("/status/{status}", post(status))
        .route("/page", get(page))
}

async fn save(
    AdminId(id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Json(setmeal): Json<SetmealDto>,
) -> ApiReturn<()> {
    server::setmeal::save(id, db, setmeal).await?;
    Ok(ApiResponse::success(()))
}

async fn page(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Query(setmeal): Query<SetmealPageQuery>,
) -> ApiReturn<Page<SetmealVo>> {
    let meals = server::setmeal::page(db, setmeal).await?;
    Ok(ApiResponse::success(meals))
}

async fn delete_meal(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Query(query): Query<QueryDelete>,
) -> ApiReturn<()> {
    let ids = query
        .ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    server::setmeal::delete(db, ids).await?;
    Ok(ApiResponse::success(()))
}

async fn get_meal(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> ApiReturn<SetmealDetailVo> {
    let meal = server::setmeal::get(db, id).await?;
    Ok(ApiResponse::success(meal))
}

async fn status(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Path(status): Path<i32>,
    Query(StateQuery { id }): Query<StateQuery>,
) -> ApiReturn<()> {
    server::setmeal::status(db, id, status).await?;
    Ok(ApiResponse::success(()))
}

async fn update(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Json(setmeal): Json<SetmealDto>,
) -> ApiReturn<()> {
    server::setmeal::update(db, setmeal).await?;
    Ok(ApiResponse::success(()))
}
