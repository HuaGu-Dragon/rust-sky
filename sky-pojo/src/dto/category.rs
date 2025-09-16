use crate::entities::category::ActiveModel;
use sea_orm::prelude::*;
use serde::{Deserialize, Deserializer};

fn string_or_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        Int(i32),
        String(String),
    }
    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::Int(i) => Ok(i),
        StringOrInt::String(s) => s.parse::<i32>().map_err(serde::de::Error::custom),
    }
}

#[derive(Debug, Clone, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDto {
    #[serde(default)]
    pub id: i64,
    pub name: String,
    #[serde(deserialize_with = "string_or_i32")]
    pub sort: i32,
    #[serde(deserialize_with = "string_or_i32")]
    pub r#type: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryUpdateDto {
    pub id: i64,
    pub name: String,
    #[serde(deserialize_with = "string_or_i32")]
    pub sort: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryQueryDto {
    pub name: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub r#type: Option<i32>,
}

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct TypeQuery {
    pub r#type: Option<i32>,
}
