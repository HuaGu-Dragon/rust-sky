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
    pub id: i64,
    pub id_number: String,
    pub name: String,
    pub phone: String,
    pub sex: String,
    pub username: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployeePageQueryDto {
    pub name: Option<String>,
    pub page: i32,
    pub page_size: i32,
}
