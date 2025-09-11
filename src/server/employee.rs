use sea_orm::{ActiveValue, IntoActiveModel, QueryTrait, prelude::*};
use sky_pojo::{
    dto::employee::{EmployeeDto, EmployeeLoginDto, EmployeePageQueryDto},
    entities::employee::{self, Model},
    vo::Page,
};
use tracing::info;

use crate::{
    server::{
        DISABLE, ENABLE,
        error::{ApiError, ApiResult},
    },
    update_params,
};

const DEFAULT_PASSWORD: &str = "123456";

pub async fn save(id: i64, db: DatabaseConnection, employee: EmployeeDto) -> ApiResult<()> {
    let mut employee = employee.into_active_model();

    employee.id = ActiveValue::NotSet;

    employee.status = ActiveValue::Set(ENABLE);

    //TODO: argon2
    employee.password = ActiveValue::Set(DEFAULT_PASSWORD.to_string());

    employee.create_user = ActiveValue::Set(Some(id));
    employee.update_user = ActiveValue::Set(Some(id));

    //TODO: error handling
    //TODO: check for duplicate usernames
    //TODO: handle the same name situation
    employee.insert(&db).await.map_err(|_| ApiError::Internal)?;
    Ok(())
}

pub async fn update(db: DatabaseConnection, employee_update: EmployeeDto) -> ApiResult<()> {
    let employee = employee::Entity::find_by_id(employee_update.id)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let mut employee = employee.into_active_model();

    update_params!(employee, id_number, employee_update.id_number);
    update_params!(employee, name, employee_update.name);
    update_params!(employee, phone, employee_update.phone);
    update_params!(employee, sex, employee_update.sex);
    update_params!(employee, username, employee_update.username);

    employee.update(&db).await.map_err(|_| ApiError::Internal)?;
    Ok(())
}

// TODO: Error handling
pub async fn get_by_id(db: DatabaseConnection, id: i64) -> ApiResult<Model> {
    let employee: Model = employee::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    Ok(employee)
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

//TODO: Error handling
pub async fn page_query(
    db: DatabaseConnection,
    EmployeePageQueryDto {
        name,
        page,
        page_size,
    }: EmployeePageQueryDto,
) -> ApiResult<Page<Model>> {
    let paginator = employee::Entity::find()
        .apply_if(name, |query, name| {
            query.filter(employee::Column::Name.contains(name))
        })
        .paginate(&db, page_size as u64);

    let num_pages = paginator.num_pages().await.unwrap();
    let employees = paginator.fetch_page(page as u64 - 1).await.unwrap();

    Ok(Page::new(num_pages as i64, employees))
}

// TODO: Error handling
pub async fn change_status(db: DatabaseConnection, id: i64, status: i32) -> ApiResult<()> {
    let employee: Model = employee::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let mut employee = employee.into_active_model();
    update_params!(employee, status, status);

    employee.update(&db).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}
