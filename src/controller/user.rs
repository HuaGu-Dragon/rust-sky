use axum::{Json, Router, routing::post};
use sky_pojo::{dto::user::UserLoginDto, vo::user::UserLoginVo};

use crate::{app::AppState, server::ApiReturn};

pub fn create_router() -> Router<AppState> {
    Router::new().route("login", post(login))
}

async fn login(Json(UserLoginDto { code }): Json<UserLoginDto>) -> ApiReturn<UserLoginVo> {
    todo!()
}
