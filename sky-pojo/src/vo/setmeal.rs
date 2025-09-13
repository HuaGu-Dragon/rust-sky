use rust_decimal::Decimal;
use sea_orm::prelude::DateTime;
use serde::Serialize;

use crate::entities::setmeal;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetmealVo {
    pub id: i64,
    pub category_id: i64,
    pub name: String,
    pub price: Decimal,
    pub status: i32,
    pub description: String,
    pub image: String,
    pub update_time: DateTime,
    pub category_name: String,
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
