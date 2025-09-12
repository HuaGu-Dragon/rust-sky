use futures_util::future::try_join_all;
use sea_orm::{ActiveValue, QueryTrait, prelude::*};
use sky_pojo::{
    dto::dish::{DishDto, DishQueryDto},
    entities::{
        category,
        dish::{self},
        dish_flavor,
    },
    vo::{
        Page,
        dish::{DishDetailVO, DishVO},
    },
};

use crate::server::error::{ApiError, ApiResult};

pub async fn save(id: i64, db: DatabaseConnection, dish: DishDto) -> ApiResult<()> {
    let (mut dish, flavors) = dish.into_active_model();

    dish.create_user = ActiveValue::Set(Some(id));
    dish.update_user = ActiveValue::Set(Some(id));

    let saved = dish.insert(&db).await.map_err(|_| ApiError::Internal)?;

    let tasks = flavors.into_iter().map(|mut flavor| {
        flavor.dish_id = ActiveValue::Set(saved.id);
        flavor.insert(&db)
    });
    try_join_all(tasks).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn get(db: DatabaseConnection, id: i64) -> ApiResult<DishDetailVO> {
    let dish = dish::Entity::find_by_id(id)
        .find_also_related(category::Entity)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let flavors = dish
        .0
        .find_related(dish_flavor::Entity)
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let dish = dish.into();
    let dish = (dish, flavors).into();

    Ok(dish)
}

pub async fn page(db: DatabaseConnection, query: DishQueryDto) -> ApiResult<Page<DishVO>> {
    let paginator = dish::Entity::find()
        .apply_if(query.category_id, |query, id| {
            query.filter(dish::Column::CategoryId.eq(id))
        })
        .apply_if(query.name, |query, name| {
            query.filter(dish::Column::Name.contains(&name))
        })
        .apply_if(query.status, |query, status| {
            query.filter(dish::Column::Status.eq(status))
        })
        .find_also_related(category::Entity)
        .paginate(&db, query.page_size as u64);

    let num_pages = paginator
        .num_pages()
        .await
        .map_err(|_| ApiError::Internal)?;
    let items = paginator
        .fetch_page((query.page - 1) as u64)
        .await
        .map_err(|_| ApiError::Internal)?;

    let items = items.into_iter().map(DishVO::from).collect();

    Ok(Page::new(num_pages as i64, items))
}
