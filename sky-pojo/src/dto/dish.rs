use crate::entities::prelude::DishFlavor;

pub struct DishDto {
    pub id: i64,
    pub name: String,
    pub price: i128,
    pub image: String,
    pub description: String,
    pub status: i32,
    pub flavors: Vec<DishFlavor>,
}
