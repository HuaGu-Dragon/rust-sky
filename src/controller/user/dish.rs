use axum::{Router, extract::State, routing::get};
use axum_extra::extract::Query;
use redis::AsyncTypedCommands;
use sky_pojo::{dto::dish::DishQueryId, vo::dish::DishDetailVO};

use crate::{
    app::AppState,
    server::{self, ApiReturn, error::ApiError, extract::UserId, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/list", get(list))
}

async fn list(
    UserId(_id): UserId,
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
