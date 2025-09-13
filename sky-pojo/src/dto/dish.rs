#![allow(clippy::needless_update)]

use sea_orm::{ActiveValue, IntoActiveModel, prelude::*};
use serde::Deserialize;

use crate::entities::dish_flavor::ActiveModel;
use crate::entities::{dish, dish_flavor};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishDto {
    #[serde(default)]
    pub id: i64,
    pub name: String,
    pub category_id: i64,
    pub price: Decimal,
    pub image: String,
    pub description: Option<String>,
    pub status: Option<i32>,
    pub flavors: Option<Vec<DishFlavorDto>>,
}

impl DishDto {
    pub fn into_active_model(self) -> (dish::ActiveModel, Vec<dish_flavor::ActiveModel>) {
        let flavor_dto = self.flavors.unwrap_or_default();

        let DishDto {
            name,
            category_id,
            price,
            image,
            description,
            status,
            ..
        } = self;

        let dish_am = dish::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(name),
            category_id: ActiveValue::Set(category_id),
            price: ActiveValue::Set(Some(price)),
            image: ActiveValue::Set(Some(image)),
            description: ActiveValue::Set(description),
            status: ActiveValue::Set(status),
            ..Default::default()
        };

        let flavor_am = flavor_dto
            .into_iter()
            .map(|f| {
                let mut model = f.into_active_model();
                model.id = ActiveValue::NotSet;
                model
            })
            .collect();

        (dish_am, flavor_am)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishQueryDto {
    pub category_id: Option<i32>,
    pub name: Option<String>,
    pub page: i32,
    pub page_size: i32,
    #[serde(deserialize_with = "super::empty_string_as_none", default)]
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishQueryId {
    pub category_id: i64,
}

#[derive(Debug, Deserialize, DeriveIntoActiveModel)]
#[serde(rename_all = "camelCase")]
pub struct DishFlavorDto {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub dish_id: i64,
    pub name: String,
    pub value: String,
}
