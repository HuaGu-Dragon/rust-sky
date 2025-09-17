use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, put},
};
use redis::AsyncTypedCommands;
use sky_pojo::{
    dto::{
        QueryDelete,
        dish::{DishDto, DishQueryDto, DishQueryId},
    },
    vo::{
        Page,
        dish::{DishDetailVO, DishVO},
    },
};

use crate::{
    app::AppState,
    server::{self, ApiReturn, error::ApiError, extract::AdminId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", put(update).delete(delete_dish).post(save))
        .route("/{id}", get(get_dish))
        .route("/page", get(page))
        .route("/list", get(list))
}

async fn save(
    AdminId(id): AdminId,
    State(AppState { db, redis }): State<AppState>,
    Json(category): Json<DishDto>,
) -> ApiReturn<()> {
    server::dish::save(id, db, redis, category).await?;
    Ok(ApiResponse::success(()))
}

async fn update(
    AdminId(id): AdminId,
    State(AppState { db, redis }): State<AppState>,
    Json(category): Json<DishDto>,
) -> ApiReturn<()> {
    server::dish::update(id, db, redis, category).await?;
    Ok(ApiResponse::success(()))
}

async fn get_dish(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> ApiReturn<DishDetailVO> {
    let dish = server::dish::get(db, id).await?;
    Ok(ApiResponse::success(dish))
}

async fn delete_dish(
    AdminId(_id): AdminId,
    State(AppState { db, redis }): State<AppState>,
    Query(query): Query<QueryDelete>,
) -> ApiReturn<()> {
    let ids = query
        .ids
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    server::dish::delete(db, redis, ids).await?;
    Ok(ApiResponse::success(()))
}

async fn page(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Query(query): Query<DishQueryDto>,
) -> ApiReturn<Page<DishVO>> {
    let dishes = server::dish::page(db, query).await?;
    Ok(ApiResponse::success(dishes))
}

async fn list(
    AdminId(_id): AdminId,
    State(AppState { db, mut redis }): State<AppState>,
    Query(DishQueryId { category_id }): Query<DishQueryId>,
) -> ApiReturn<Vec<DishDetailVO>> {
    let key = format!("dish_{category_id}");
    if let Ok(Some(cached)) = redis.get(&key).await
        && let Ok(dishes) = serde_json::from_str::<Vec<DishDetailVO>>(&cached)
    {
        Ok(ApiResponse::success(dishes))
    } else {
        let dishes = server::dish::list(db, category_id).await?;
        redis
            .set(
                key,
                serde_json::to_string(&dishes).map_err(|_| ApiError::Internal)?,
            )
            .await
            .map_err(|_| ApiError::Internal)?;

        Ok(ApiResponse::success(dishes))
    }
}
