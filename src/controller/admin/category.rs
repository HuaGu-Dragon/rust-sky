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
    server::{self, ApiReturn, extract::AdminId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", put(update).delete(delete_category).post(save))
        .route("/status/{status}", post(status))
        .route("/list", get(list))
        .route("/page", get(page))
}

async fn save(
    AdminId(id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Json(category): Json<CategoryDto>,
) -> ApiReturn<()> {
    server::category::save(id, db, category).await?;
    Ok(ApiResponse::success(()))
}

async fn update(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Json(category): Json<CategoryUpdateDto>,
) -> ApiReturn<()> {
    server::category::update(db, category).await?;
    Ok(ApiResponse::success(()))
}

async fn delete_category(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Query(category): Query<IdQuery>,
) -> ApiReturn<()> {
    server::category::delete(db, category.id).await?;
    Ok(ApiResponse::success(()))
}

async fn status(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Path(status): Path<i32>,
    Query(category): Query<IdQuery>,
) -> ApiReturn<()> {
    server::category::status(db, category.id, status).await?;
    Ok(ApiResponse::success(()))
}

async fn list(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Query(category): Query<TypeQuery>,
) -> ApiReturn<Vec<Model>> {
    let categories = server::category::list(db, category.r#type).await?;
    Ok(ApiResponse::success(categories))
}

async fn page(
    AdminId(_id): AdminId,
    State(AppState { db, .. }): State<AppState>,
    Query(category): Query<CategoryQueryDto>,
) -> ApiReturn<Page<Model>> {
    let categories = server::category::page(db, category).await?;
    Ok(ApiResponse::success(categories))
}
