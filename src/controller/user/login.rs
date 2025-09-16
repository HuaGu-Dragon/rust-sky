use axum::{Json, Router, extract::State, routing::post};
use sky_pojo::{dto::user::UserLoginDto, vo::user::UserLoginVo};

use crate::{
    app::AppState,
    server::{
        self, ApiReturn,
        auth::{JwtAuthKey, jwt_service},
        response::ApiResponse,
    },
};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(AppState { db, .. }): State<AppState>,
    Json(UserLoginDto { code }): Json<UserLoginDto>,
) -> ApiReturn<UserLoginVo> {
    let user = server::user::login(db, code).await?;

    let user = UserLoginVo {
        id: user.id,
        openid: user.openid.unwrap(),
        token: jwt_service().encode(JwtAuthKey::UserId, user.id)?,
    };

    Ok(ApiResponse::success(user))
}
