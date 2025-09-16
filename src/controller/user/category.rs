use axum::{Router, extract::State, routing::get};
use axum_extra::extract::Query;
use sky_pojo::{dto::category::TypeQuery, entities::category::Model};

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
    Query(category): Query<TypeQuery>,
) -> ApiReturn<Vec<Model>> {
    let categories = server::category::list(db, category.r#type).await?;
    Ok(ApiResponse::success(categories))
}
