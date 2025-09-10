use axum::{Json, Router, extract::State, routing::post};
use sky_pojo::{
    dto::employee::{EmployeeDto, EmployeeLoginDto},
    vo::employee::EmployeeLoginVO,
};
use tracing::info;

use crate::{
    app::AppState,
    server::{self, ApiReturn, auth, extract::Id, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(save))
        .route("/login", post(login))
}

async fn save(
    Id(id): Id,
    State(AppState { db }): State<AppState>,
    Json(employee): Json<EmployeeDto>,
) -> ApiReturn<()> {
    info!("Add new employee");
    server::employee::save(id, db, employee).await?;

    Ok(ApiResponse::success(()))
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
        token: auth::jwt_service().encode(employee.id)?,
    };

    info!("Login successful for user: {}", employee.user_name);

    Ok(ApiResponse::success(employee))
}
