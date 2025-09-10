use crate::entities::employee::ActiveModel;
use sea_orm::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmployeeLoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct EmployeeDto {
    #[serde(default)]
    id: i64,
    id_number: String,
    name: String,
    phone: String,
    sex: String,
    username: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeePageQueryDto {
    pub name: Option<String>,
    pub page: i32,
    pub page_size: i32,
}

#[derive(Debug, Deserialize)]
pub struct StateQuery {
    pub id: i64,
}
