use rust_decimal::Decimal;
use sea_orm::prelude::DateTime;
use serde::Serialize;

use crate::entities::{category, dish};

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

impl From<(dish::Model, Option<category::Model>)> for DishVO {
    fn from((dish, category): (dish::Model, Option<category::Model>)) -> Self {
        Self {
            id: dish.id,
            name: dish.name,
            category_id: dish.category_id,
            price: dish.price.unwrap_or_default(),
            image: dish.image.unwrap_or_default(),
            description: dish.description.unwrap_or_default(),
            status: dish.status.unwrap_or_default(),
            update_time: dish.update_time,
            category_name: category.map(|c| c.name).unwrap_or_default(),
        }
    }
}
