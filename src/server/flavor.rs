use sea_orm::prelude::*;
use sky_pojo::{entities::dish_flavor, vo::flavor::DishFlavorVO};

use crate::server::error::{ApiError, ApiResult};

// TODO: add save function, without unwrapping the result of into_active_model
pub async fn get(db: DatabaseConnection, id: i64) -> ApiResult<Vec<DishFlavorVO>> {
    let flavors = dish_flavor::Entity::find()
        .filter(dish_flavor::Column::DishId.eq(id))
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let flavors = flavors
        .into_iter()
        .map(|m| DishFlavorVO {
            dish_id: m.dish_id,
            id: m.id,
            name: m.name.unwrap_or_default(),
            value: m.value.unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    Ok(flavors)
}
