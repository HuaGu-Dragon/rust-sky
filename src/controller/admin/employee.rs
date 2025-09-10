use axum::{Json, Router, extract::State, routing::post};
use sky_pojo::{dto::employee::EmployeeLoginDto, vo::employee::EmployeeLoginVO};
use tracing::info;

use crate::{
    app::AppState,
    server::{self, ApiReturn, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(AppState { db }): State<AppState>,
    Json(employee): Json<EmployeeLoginDto>,
) -> ApiReturn<EmployeeLoginVO> {
    let employee = server::employee::login(db, employee).await?;

    let employee = EmployeeLoginVO {
        id: employee.id,
        user_name: employee.username,
        name: employee.name,
        token: "token".to_string(), // TODO: Generate JWT token
    };

    info!("Login successful for user: {}", employee.user_name);

    Ok(ApiResponse::success(employee))
}
