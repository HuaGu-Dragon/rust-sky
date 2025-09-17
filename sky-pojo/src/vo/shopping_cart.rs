use rust_decimal::Decimal;
use sea_orm::prelude::DateTime;
use serde::Serialize;

use crate::entities::shopping_cart;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CartVO {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub dish_id: Option<i64>,
    pub setmeal_id: Option<i64>,
    pub dish_flavor: String,
    pub number: i32,
    pub amount: Decimal,
    pub image: String,
    pub create_time: DateTime,
}

impl From<shopping_cart::Model> for CartVO {
    fn from(value: shopping_cart::Model) -> Self {
        Self {
            id: value.id,
            name: value.name.unwrap_or_default(),
            user_id: value.user_id,
            dish_id: value.dish_id,
            setmeal_id: value.setmeal_id,
            dish_flavor: value.dish_flavor.unwrap_or_default(),
            number: value.number,
            amount: value.amount,
            image: value.image.unwrap_or_default(),
            create_time: value.create_time.unwrap_or_default(),
        }
    }
}
