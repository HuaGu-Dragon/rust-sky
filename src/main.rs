use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use sea_orm::{EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use sky_pojo::dto::employee;

use crate::app::AppState;

mod app;
mod config;
mod database;
mod logger;
mod server;

use sea_orm::ColumnTrait;

#[tokio::main]
async fn main() {
    app::run(Router::new().nest(
        "/admin",
        Router::new().nest("/employee", Router::new().route("/login", post(login))),
    ))
    .await;
}

async fn login(
    State(AppState { db }): State<AppState>,
    Json(employee): Json<sky_pojo::dto::employee::EmployeeLoginDto>,
) -> impl IntoResponse {
    println!(
        "username: {}, password: {}",
        employee.username, employee.password
    );

    let employee_find = sky_pojo::entities::employee::Entity::find()
        .filter(sky_pojo::entities::employee::Column::Username.eq(employee.username))
        .one(&db)
        .await
        .unwrap();

    // if employee_find.is_none() {
    //     println!("User does not exist");
    //     return;
    // }

    let employee_find = employee_find.unwrap();

    // if employee_find.password != employee.password {
    //     println!("Incorrect password");
    //     return;
    // }

    let employee = sky_pojo::vo::employee::EmployeeLoginVO {
        id: employee_find.id,
        username: employee_find.username,
        name: employee_find.name,
        token: "token".to_string(), // TODO: Generate JWT token
    };

    println!("Login successful");

    Ret {
        code: 1,
        msg: "Login successful".to_string(),
        data: employee,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ret<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T: Serialize> IntoResponse for Ret<T> {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, Json(self)).into_response()
    }
}
