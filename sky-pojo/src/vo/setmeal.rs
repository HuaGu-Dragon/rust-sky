use rust_decimal::Decimal;
use sea_orm::{ActiveValue::Set, prelude::DateTime};
use serde::Serialize;

use crate::entities::{setmeal, setmeal_dish};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetmealVo {
    pub id: i64,
    pub category_id: i64,
    pub name: String,
    #[serde(with = "rust_decimal::serde::float")]
    pub price: Decimal,
    pub status: i32,
    pub description: String,
    pub image: String,
    pub update_time: DateTime,
    pub category_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetmealDetailVo {
    #[serde(flatten)]
    pub setmeal: SetmealVo,
    pub setmeal_dishes: Vec<SetmealDishVo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetmealDishVo {
    copies: i32,
    dish_id: i64,
    id: i64,
    name: String,
    #[serde(with = "rust_decimal::serde::float")]
    price: Decimal,
    setmeal_id: i64,
}

//TODO: remove unwrap_or_default, handle Option properly
impl From<(setmeal::Model, Option<crate::entities::category::Model>)> for SetmealVo {
    fn from(
        (setmeal, category): (setmeal::Model, Option<crate::entities::category::Model>),
    ) -> Self {
        Self {
            id: setmeal.id,
            category_id: setmeal.category_id,
            name: setmeal.name,
            price: setmeal.price,
            status: setmeal.status.unwrap_or_default(),
            description: setmeal.description.unwrap_or_default(),
            image: setmeal.image.unwrap_or_default(),
            update_time: setmeal.update_time.unwrap_or_default(),
            category_name: category.map(|c| c.name).unwrap_or_default(),
        }
    }
}

impl From<setmeal_dish::Model> for SetmealDishVo {
    fn from(value: setmeal_dish::Model) -> Self {
        Self {
            copies: value.copies.unwrap_or_default(),
            dish_id: value.dish_id.unwrap_or_default(),
            id: value.id,
            name: value.name.unwrap_or_default(),
            price: value.price.unwrap_or_default(),
            setmeal_id: value.setmeal_id.unwrap_or_default(),
        }
    }
}

impl From<(SetmealVo, Vec<setmeal_dish::Model>)> for SetmealDetailVo {
    fn from((setmeal, dishes): (SetmealVo, Vec<setmeal_dish::Model>)) -> Self {
        Self {
            setmeal,
            setmeal_dishes: dishes.into_iter().map(SetmealDishVo::from).collect(),
        }
    }
}
