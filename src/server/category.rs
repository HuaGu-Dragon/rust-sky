use sea_orm::{ActiveValue, IntoActiveModel, QueryOrder, QueryTrait, prelude::*};
use sky_pojo::{
    dto::category::{CategoryDto, CategoryQueryDto, CategoryUpdateDto},
    entities::category::{self, Model},
    vo::Page,
};

use crate::{
    server::{
        ENABLE,
        error::{ApiError, ApiResult},
    },
    update_params,
};

pub async fn save(id: i64, db: DatabaseConnection, category: CategoryDto) -> ApiResult<()> {
    let mut category = category.into_active_model();

    category.id = ActiveValue::NotSet;

    category.status = ActiveValue::Set(Some(ENABLE));

    category.create_user = ActiveValue::Set(Some(id));
    category.update_user = ActiveValue::Set(Some(id));

    category.insert(&db).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn update(db: DatabaseConnection, category_update: CategoryUpdateDto) -> ApiResult<()> {
    let mut category = category::Entity::find_by_id(category_update.id)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?
        .into_active_model();

    update_params!(category, name, category_update.name);
    update_params!(category, sort, category_update.sort);
    // update_params!(
    //     category,
    //     update_time,
    //     Some(chrono::Utc::now().naive_local())
    // );

    category.update(&db).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn delete(db: DatabaseConnection, id: i64) -> ApiResult<()> {
    let category = category::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    category.delete(&db).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn status(db: DatabaseConnection, id: i64, status: i32) -> ApiResult<()> {
    let category = category::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let mut category = category.into_active_model();
    update_params!(category, status, Some(status));

    category.update(&db).await.map_err(|_| ApiError::Internal)?;
    Ok(())
}

pub async fn list(db: DatabaseConnection, r#type: Option<i32>) -> ApiResult<Vec<Model>> {
    let categories = category::Entity::find()
        .apply_if(r#type, |query, r#type| {
            query.filter(category::Column::Type.eq(r#type))
        })
        .order_by_asc(category::Column::Sort)
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    Ok(categories)
}

pub async fn page(
    db: DatabaseConnection,
    CategoryQueryDto {
        name,
        page,
        page_size,
        r#type,
    }: CategoryQueryDto,
) -> ApiResult<Page<Model>> {
    let paginator = category::Entity::find()
        .apply_if(name, |query, name| {
            query.filter(category::Column::Name.contains(name))
        })
        .apply_if(r#type, |query, t| {
            query.filter(category::Column::Type.eq(t))
        })
        .order_by_asc(category::Column::Sort)
        .paginate(&db, page_size as u64);

    let num_pages = paginator
        .num_pages()
        .await
        .map_err(|_| ApiError::Internal)?;
    let categories = paginator
        .fetch_page((page - 1) as u64)
        .await
        .map_err(|_| ApiError::Internal)?;

    Ok(Page::new(num_pages as i64, categories))
}
