use serde::Serialize;

use crate::entities::dish_flavor;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DishFlavorVO {
    pub dish_id: i64,
    pub id: i64,
    pub name: String,
    pub value: String,
}

impl From<dish_flavor::Model> for DishFlavorVO {
    fn from(value: dish_flavor::Model) -> Self {
        Self {
            dish_id: value.dish_id,
            id: value.id,
            name: value.name.unwrap_or_default(),
            value: value.value.unwrap_or_default(),
        }
    }
}
