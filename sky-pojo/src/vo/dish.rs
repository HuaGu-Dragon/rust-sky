use rust_decimal::Decimal;
use sea_orm::prelude::DateTime;
use serde::Serialize;

use crate::entities::dish;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DishVO {
    pub id: i64,
    pub name: String,
    pub category_id: i64,
    #[serde(with = "rust_decimal::serde::float")]
    pub price: Decimal,
    pub image: String,
    pub description: String,
    pub status: i32,
    pub update_time: Option<DateTime>,
    pub category_name: String,
}

impl From<dish::Model> for DishVO {
    fn from(value: dish::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            category_id: value.category_id,
            price: value.price.unwrap_or_default(),
            image: value.image.unwrap_or_default(),
            description: value.description.unwrap_or_default(),
            status: value.status.unwrap_or_default(),
            update_time: value.update_time,
            category_name: "".to_string(), // Placeholder, should be set appropriately
        }
    }
}
