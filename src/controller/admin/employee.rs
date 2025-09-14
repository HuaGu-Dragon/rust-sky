use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use sky_pojo::{
    dto::{
        StateQuery,
        employee::{EmployeeDto, EmployeeLoginDto, EmployeePageQueryDto},
    },
    entities::employee::Model,
    vo::{Page, employee::EmployeeLoginVO},
};
use tracing::info;

use crate::{
    app::AppState,
    server::{self, ApiReturn, auth, extract::Id, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(save).put(update))
        .route("/{id}", get(get_employee))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/page", get(page))
        .route("/status/{status}", post(status))
}

async fn save(
    Id(id): Id,
    State(AppState { db, .. }): State<AppState>,
    Json(employee): Json<EmployeeDto>,
) -> ApiReturn<()> {
    info!("Add new employee");
    server::employee::save(id, db, employee).await?;

    Ok(ApiResponse::success(()))
}

async fn update(
    Id(_id): Id,
    State(AppState { db, .. }): State<AppState>,
    Json(employee): Json<EmployeeDto>,
) -> ApiReturn<()> {
    info!("Update employee information");
    server::employee::update(db, employee).await?;

    Ok(ApiResponse::success(()))
}

async fn get_employee(
    Id(_id): Id,
    State(AppState { db, .. }): State<AppState>,
    Path(id): Path<i64>,
) -> ApiReturn<Model> {
    let employee = server::employee::get_by_id(db, id).await?;
    Ok(ApiResponse::success(employee))
}

async fn login(
    State(AppState { db, .. }): State<AppState>,
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

async fn logout() -> ApiReturn<()> {
    info!("Logout successful");
    Ok(ApiResponse::success(()))
}

async fn page(
    Id(_id): Id,
    State(AppState { db, .. }): State<AppState>,
    Query(employee): Query<EmployeePageQueryDto>,
) -> ApiReturn<Page<Model>> {
    let employees = server::employee::page_query(db, employee).await?;

    Ok(ApiResponse::success(employees))
}

async fn status(
    Id(_id): Id,
    State(AppState { db, .. }): State<AppState>,
    Path(status): Path<i32>,
    Query(StateQuery { id }): Query<StateQuery>,
) -> ApiReturn<()> {
    info!("Change employee {id} status to {status}");
    server::employee::change_status(db, id, status).await?;

    Ok(ApiResponse::success(()))
}
