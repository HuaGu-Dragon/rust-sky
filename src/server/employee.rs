use sea_orm::{ActiveValue, IntoActiveModel, prelude::*};
use sky_pojo::{
    dto::employee::{EmployeeDto, EmployeeLoginDto},
    entities::employee::{self, Model},
};
use sqlx::types::chrono;
use tracing::info;

use crate::server::error::{ApiError, ApiResult};

const ENABLE: i32 = 1;
const DISABLE: i32 = 0;

const DEFAULT_PASSWORD: &str = "123456";

pub async fn save(id: i64, db: DatabaseConnection, employee: EmployeeDto) -> ApiResult<()> {
    let mut employee = employee.into_active_model();

    employee.id = ActiveValue::NotSet;

    employee.status = ActiveValue::Set(ENABLE);

    //TODO: argon2
    employee.password = ActiveValue::Set(DEFAULT_PASSWORD.to_string());

    //TODO: change to the model_injection method
    employee.create_time = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
    employee.update_time = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

    employee.create_user = ActiveValue::Set(Some(id));
    employee.update_user = ActiveValue::Set(Some(id));

    //TODO: error handling
    employee.insert(&db).await.map_err(|_| ApiError::Internal)?;
    Ok(())
}

pub async fn login(
    db: DatabaseConnection,
    EmployeeLoginDto { username, password }: EmployeeLoginDto,
) -> ApiResult<Model> {
    let employee = employee::Entity::find()
        .filter(employee::Column::Username.eq(username))
        .one(&db)
        .await
        .map_err(|_| ApiError::LoginError)?;

    let employee = employee.ok_or(ApiError::LoginError)?;

    // TODO: argon2 password verification
    if employee.status == DISABLE {
        Err(ApiError::AccountLocked)
    } else if employee.password != password {
        Err(ApiError::LoginError)
    } else {
        info!("Login successful");
        Ok(employee)
    }
}
