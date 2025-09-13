use rust_decimal::Decimal;
use sea_orm::{ActiveValue, DeriveIntoActiveModel, IntoActiveModel};
use serde::Deserialize;

use crate::entities::setmeal;
use crate::entities::setmeal_dish::ActiveModel;

#[allow(unused)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetmealDto {
    category_id: i64,
    description: Option<String>,
    id: Option<i64>,
    image: String,
    name: String,
    price: Decimal,
    setmeal_dishes: Vec<SetmealDishDto>,
    status: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetmealPageQuery {
    pub category_id: Option<i64>,
    pub name: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub status: Option<i32>,
}

#[derive(Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct SetmealDishDto {
    copies: i32,
    dish_id: i64,
    name: String,
    price: Decimal,
    setmeal_id: i64,
}

impl SetmealDto {
    pub fn into_active_model(self) -> (setmeal::ActiveModel, Vec<ActiveModel>) {
        let setmeal_dish_dto = self.setmeal_dishes;
        let SetmealDto {
            category_id,
            description,
            image,
            name,
            price,
            status,
            ..
        } = self;
        let setmeal_am = setmeal::ActiveModel {
            id: ActiveValue::NotSet,
            category_id: ActiveValue::Set(category_id),
            name: ActiveValue::Set(name),
            price: ActiveValue::Set(price),
            status: ActiveValue::Set(Some(status)),
            description: ActiveValue::Set(description),
            image: ActiveValue::Set(Some(image)),
            create_time: ActiveValue::NotSet,
            update_time: ActiveValue::NotSet,
            create_user: ActiveValue::NotSet,
            update_user: ActiveValue::NotSet,
        };

        let setmeal_dish_am = setmeal_dish_dto
            .into_iter()
            .map(|f| {
                let mut model = f.into_active_model();
                model.id = ActiveValue::NotSet;
                model
            })
            .collect::<Vec<ActiveModel>>();

        (setmeal_am, setmeal_dish_am)
    }
}
