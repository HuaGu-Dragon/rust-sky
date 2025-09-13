use futures_util::future::try_join_all;
use sea_orm::{QueryTrait, prelude::*};
use sky_pojo::{
    dto::setmeal::{SetmealDto, SetmealPageQuery},
    entities::{category, setmeal},
    vo::{Page, setmeal::SetmealVo},
};

use crate::server::error::{ApiError, ApiResult};

pub async fn save(id: i64, db: DatabaseConnection, setmeal: SetmealDto) -> ApiResult<()> {
    let (mut setmeal, setmeal_dishes) = setmeal.into_active_model();

    setmeal.create_user = sea_orm::ActiveValue::Set(Some(id));
    setmeal.update_user = sea_orm::ActiveValue::Set(Some(id));

    let saved = setmeal.insert(&db).await.map_err(|_| ApiError::Internal)?;

    let tasks = setmeal_dishes.into_iter().map(|mut sd| {
        sd.setmeal_id = sea_orm::ActiveValue::Set(Some(saved.id));
        sd.insert(&db)
    });
    try_join_all(tasks).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn page(db: DatabaseConnection, query: SetmealPageQuery) -> ApiResult<Page<SetmealVo>> {
    let paginator = setmeal::Entity::find()
        .apply_if(query.category_id, |query, category_id| {
            query.filter(setmeal::Column::CategoryId.eq(category_id))
        })
        .apply_if(query.name, |query, name| {
            query.filter(setmeal::Column::Name.contains(&name))
        })
        .apply_if(query.status, |query, status| {
            query.filter(setmeal::Column::Status.eq(status))
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

    let items = items.into_iter().map(SetmealVo::from).collect();
    Ok(Page::new(num_pages as i64, items))
}
