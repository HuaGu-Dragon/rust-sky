#![allow(clippy::needless_update)]

use sea_orm::{ActiveValue, IntoActiveModel, prelude::*};
use serde::{
    Deserialize, Deserializer,
    de::{self, Unexpected},
};

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
    #[serde(deserialize_with = "empty_string_as_none", default)]
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishQueryDelete {
    pub ids: String,
}

// uri=/admin/dish/page?page=1&pageSize=10&status=
// what is that fucking request uri???
fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Opt {
        I(i32),
        S(String),
        N,
    }

    match Opt::deserialize(deserializer)? {
        Opt::I(i) => Ok(Some(i)),
        Opt::S(s) if s.trim().is_empty() => Ok(None),
        Opt::S(s) => s
            .parse::<i32>()
            .map(Some)
            .map_err(|_| de::Error::invalid_value(Unexpected::Str(&s), &"an integer or empty")),
        Opt::N => Ok(None),
    }
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
