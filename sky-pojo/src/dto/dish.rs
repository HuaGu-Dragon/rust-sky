use crate::entities::dish::ActiveModel;
use sea_orm::{ActiveValue, IntoActiveModel, prelude::*};
use serde::{
    Deserialize, Deserializer,
    de::{self, Unexpected},
};

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
    #[serde(deserialize_with = "empty_string_as_none", default)]
    pub status: Option<i32>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DishFlavorDto {
    pub id: Option<i64>,
    pub dish_id: Option<i64>,
    pub name: String,
    pub value: String,
}
