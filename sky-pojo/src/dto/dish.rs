use crate::entities::dish::ActiveModel;
use sea_orm::{ActiveValue, IntoActiveModel, prelude::*};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishDto {
    pub id: Option<i64>,
    pub name: String,
    pub category_id: i64,
    pub price: Decimal,
    pub image: String,
    pub description: Option<String>,
    pub status: Option<i32>,
    pub flavors: Option<Vec<DishFlavorDto>>,
}

impl IntoActiveModel<ActiveModel> for DishDto {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            id: if let Some(id) = self.id {
                ActiveValue::Set(id)
            } else {
                ActiveValue::NotSet
            },
            name: ActiveValue::Set(self.name),
            category_id: ActiveValue::Set(self.category_id),
            price: ActiveValue::Set(Some(self.price)),
            image: ActiveValue::Set(Some(self.image)),
            description: ActiveValue::Set(self.description),
            status: ActiveValue::Set(self.status),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishQueryDto {
    pub category_id: Option<i32>,
    pub name: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishFlavorDto {
    pub id: Option<i64>,
    pub dish_id: Option<i64>,
    pub name: String,
    pub value: String,
}
