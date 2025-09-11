use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post, put},
};
use sky_pojo::{
    dto::category::{CategoryDto, CategoryQueryDto, CategoryUpdateDto, IdQuery, TypeQuery},
    entities::category::Model,
    vo::Page,
};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::Id, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", put(update).delete(delete_category).post(save))
        .route("/status/{status}", post(status))
        .route("/list", get(list))
        .route("/page", get(page))
}

async fn save(
    Id(id): Id,
    State(AppState { db }): State<AppState>,
    Json(category): Json<CategoryDto>,
) -> ApiReturn<()> {
    server::category::save(id, db, category).await?;
    Ok(ApiResponse::success(()))
}

async fn update(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
    Json(category): Json<CategoryUpdateDto>,
) -> ApiReturn<()> {
    server::category::update(db, category).await?;
    Ok(ApiResponse::success(()))
}

async fn delete_category(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
    Query(category): Query<IdQuery>,
) -> ApiReturn<()> {
    server::category::delete(db, category.id).await?;
    Ok(ApiResponse::success(()))
}

async fn status(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
    Path(status): Path<i32>,
    Query(category): Query<IdQuery>,
) -> ApiReturn<()> {
    server::category::status(db, category.id, status).await?;
    Ok(ApiResponse::success(()))
}

async fn list(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
    Query(category): Query<TypeQuery>,
) -> ApiReturn<Page<Model>> {
    let categories = server::category::list(db, category.r#type).await?;
    Ok(ApiResponse::success(categories))
}

async fn page(
    Id(_id): Id,
    State(AppState { db }): State<AppState>,
    Query(category): Query<CategoryQueryDto>,
) -> ApiReturn<Page<Model>> {
    let categories = server::category::page(db, category).await?;
    Ok(ApiResponse::success(categories))
}
