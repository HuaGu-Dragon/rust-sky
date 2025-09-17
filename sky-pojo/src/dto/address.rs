#![allow(clippy::needless_update)]

use crate::entities::address_book::ActiveModel;
use sea_orm::DeriveIntoActiveModel;
use serde::Deserialize;

#[derive(Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct AddressDto {
    #[serde(default)]
    pub id: i64,
    pub consignee: Option<String>,
    pub sex: String,
    pub phone: String,
    pub province_code: Option<String>,
    pub province_name: Option<String>,
    pub city_code: Option<String>,
    pub city_name: Option<String>,
    pub district_code: Option<String>,
    pub district_name: Option<String>,
    pub detail: String,
    #[serde(deserialize_with = "super::integer_as_string")]
    pub label: String,
    #[serde(default)]
    pub is_default: i16,
}
