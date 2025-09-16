use axum::{Router, extract::State, routing::get};
use axum_extra::extract::Query;
use sky_pojo::{dto::dish::DishQueryId, vo::dish::DishVO};

use crate::{
    app::AppState,
    server::{self, ApiReturn, extract::UserId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/list", get(list))
}

async fn list(
    UserId(_id): UserId,
    State(AppState { db, .. }): State<AppState>,
    Query(query): Query<DishQueryId>,
) -> ApiReturn<Vec<DishVO>> {
    let dishes = server::dish::list(db, query.category_id).await?;
    Ok(ApiResponse::success(dishes))
}
