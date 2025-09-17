use sea_orm::{ActiveValue, IntoActiveModel, prelude::*};
use sky_pojo::{dto::address::AddressDto, entities::address_book};

use crate::server::error::{ApiError, ApiResult};

pub async fn add(id: i64, db: DatabaseConnection, address: AddressDto) -> ApiResult<()> {
    let mut address = address.into_active_model();

    address.id = ActiveValue::NotSet;
    address.user_id = ActiveValue::Set(id);

    address.insert(&db).await.map_err(|_| ApiError::Internal)?;
    Ok(())
}

pub async fn list(
    id: i64,
    db: DatabaseConnection,
) -> ApiResult<Vec<sky_pojo::vo::address::AddressVO>> {
    let addresses = address_book::Entity::find()
        .filter(address_book::Column::UserId.eq(id))
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let addresses = addresses
        .into_iter()
        .map(sky_pojo::vo::address::AddressVO::from)
        .collect();

    Ok(addresses)
}
