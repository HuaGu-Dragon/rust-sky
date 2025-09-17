use sea_orm::{ActiveValue, IntoActiveModel, prelude::*};
use sky_pojo::{dto::address::AddressDto, entities::address_book, vo::address::AddressVO};

use crate::{
    server::{
        DISABLE, ENABLE,
        error::{ApiError, ApiResult},
    },
    update_params,
};

pub async fn add(id: i64, db: DatabaseConnection, address: AddressDto) -> ApiResult<()> {
    let mut address = address.into_active_model();

    address.id = ActiveValue::NotSet;
    address.user_id = ActiveValue::Set(id);

    address.insert(&db).await.map_err(|_| ApiError::Internal)?;
    Ok(())
}

pub async fn list(id: i64, db: DatabaseConnection) -> ApiResult<Vec<AddressVO>> {
    let addresses = address_book::Entity::find()
        .filter(address_book::Column::UserId.eq(id))
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let addresses = addresses.into_iter().map(AddressVO::from).collect();

    Ok(addresses)
}

pub async fn default_address(id: i64, db: DatabaseConnection) -> ApiResult<Option<AddressVO>> {
    let address = address_book::Entity::find()
        .filter(address_book::Column::UserId.eq(id))
        .filter(address_book::Column::IsDefault.eq(ENABLE))
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    Ok(address.map(AddressVO::from))
}

pub async fn remove(address_id: i64, db: DatabaseConnection) -> ApiResult<()> {
    let address = address_book::Entity::find()
        .filter(address_book::Column::Id.eq(address_id))
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    address.delete(&db).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn get(address_id: i64, db: DatabaseConnection) -> ApiResult<AddressVO> {
    let address = address_book::Entity::find()
        .filter(address_book::Column::Id.eq(address_id))
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    Ok(AddressVO::from(address))
}

pub async fn set_default(address_id: i64, db: DatabaseConnection) -> ApiResult<()> {
    let addresses = address_book::Entity::find()
        .filter(address_book::Column::IsDefault.eq(ENABLE))
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    for address in addresses {
        let mut address = address.into_active_model();
        address.is_default = ActiveValue::Set(DISABLE as i16);
        address.update(&db).await.map_err(|_| ApiError::Internal)?;
    }

    let address = address_book::Entity::find()
        .filter(address_book::Column::Id.eq(address_id))
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let mut address = address.into_active_model();
    address.is_default = ActiveValue::Set(ENABLE as i16);
    address.update(&db).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn update(address: AddressDto, db: DatabaseConnection) -> ApiResult<()> {
    let existing = address_book::Entity::find()
        .filter(address_book::Column::Id.eq(address.id))
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let mut model = existing.into_active_model();
    update_params!(model, consignee, address.consignee);
    update_params!(model, sex, Some(address.sex));
    update_params!(model, phone, address.phone);
    update_params!(model, province_code, address.province_code);
    update_params!(model, province_name, address.province_name);
    update_params!(model, city_code, address.city_code);
    update_params!(model, city_name, address.city_name);
    update_params!(model, district_code, address.district_code);
    update_params!(model, district_name, address.district_name);
    update_params!(model, detail, Some(address.detail));
    update_params!(model, label, Some(address.label));

    model.update(&db).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}
