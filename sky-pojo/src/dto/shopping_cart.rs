use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CartDto {
    pub dish_flavor: Option<String>,
    pub dish_id: Option<i64>,
    pub setmeal_id: Option<i64>,
}
