use sea_orm::prelude::*;
use sky_pojo::{
    dto::employee::EmployeeLoginDto,
    entities::employee::{self, Model},
};

use crate::server::error::{ApiError, ApiResult};

// TODO: error handling
pub async fn login(
    db: DatabaseConnection,
    EmployeeLoginDto { username, password }: EmployeeLoginDto,
) -> ApiResult<Model> {
    let employee = employee::Entity::find()
        .filter(employee::Column::Username.eq(username))
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    // TODO: handle user not found

    let employee = employee.unwrap();

    // TODO: argon2 password verification
    if employee.password == password {
        println!("Login successful");
    } else {
        println!("Incorrect password");
    }

    Ok(employee)
}
