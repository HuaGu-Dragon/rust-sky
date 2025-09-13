use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use sky_pojo::{
    dto::{
        QueryDelete,
        setmeal::{SetmealDto, SetmealPageQuery},
    },
    vo::{Page, setmeal::SetmealVo},
};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::Id, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(save).delete(delete_meal))
        .route("/page", get(page))
}

async fn save(
    Id(id): Id,
    State(AppState { db }): State<AppState>,
    Json(setmeal): Json<SetmealDto>,
) -> ApiReturn<()> {
    server::setmeal::save(id, db, setmeal).await?;
    Ok(ApiResponse::success(()))
}

async fn page(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
    Query(setmeal): Query<SetmealPageQuery>,
) -> ApiReturn<Page<SetmealVo>> {
    let meals = server::setmeal::page(db, setmeal).await?;
    Ok(ApiResponse::success(meals))
}

async fn delete_meal(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
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
