use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{get, post},
};
use sky_pojo::{
    dto::dish::{DishDto, DishQueryDto},
    vo::{Page, dish::DishVO},
};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::Id, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(save))
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

async fn page(
    State(AppState { db }): State<AppState>,
    Query(query): Query<DishQueryDto>,
) -> ApiReturn<Page<DishVO>> {
    let dishes = server::dish::page(db, query).await?;
    Ok(ApiResponse::success(dishes))
}
