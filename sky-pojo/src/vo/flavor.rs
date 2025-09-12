use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DishFlavorVO {
    pub dish_id: i64,
    pub id: i64,
    pub name: String,
    pub value: String,
}
