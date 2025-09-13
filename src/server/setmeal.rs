use futures_util::{future::try_join_all, try_join};
use sea_orm::{ActiveValue, Condition, IntoActiveModel, QueryTrait, TransactionTrait, prelude::*};
use sky_pojo::{
    dto::setmeal::{SetmealDto, SetmealPageQuery},
    entities::{category, setmeal, setmeal_dish},
    vo::{
        Page,
        setmeal::{SetmealDetailVo, SetmealVo},
    },
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

pub async fn delete(db: DatabaseConnection, ids: Vec<i64>) -> ApiResult<()> {
    let meals = setmeal::Entity::find()
        .filter(
            Condition::all()
                .add(setmeal::Column::Id.is_in(ids))
                .add(setmeal::Column::Status.eq(0)),
        )
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    if meals.is_empty() {
        return Err(ApiError::BadRequest(
            "Some meals are currently on sale".to_string(),
        ));
    }

    let valid_ids: Vec<i64> = meals.iter().map(|m| m.id).collect();

    let txn = db.begin().await.map_err(|_| ApiError::Internal)?;

    setmeal_dish::Entity::delete_many()
        .filter(setmeal_dish::Column::SetmealId.is_in(valid_ids.clone()))
        .exec(&txn)
        .await
        .map_err(|_| ApiError::Internal)?;

    setmeal::Entity::delete_many()
        .filter(setmeal::Column::Id.is_in(valid_ids))
        .exec(&txn)
        .await
        .map_err(|_| ApiError::Internal)?;

    txn.commit().await.map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn get(db: DatabaseConnection, id: i64) -> ApiResult<SetmealDetailVo> {
    let meal = setmeal::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let (category, dishes) = try_join!(
        meal.find_related(category::Entity).one(&db),
        meal.find_related(setmeal_dish::Entity).all(&db)
    )
    .map_err(|_| ApiError::Internal)?;

    let meal = (meal, category).into();
    let meal = (meal, dishes).into();

    Ok(meal)
}

pub async fn status(db: DatabaseConnection, id: i64, status: i32) -> ApiResult<()> {
    let meal = setmeal::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let mut meal = meal.into_active_model();

    meal.status = ActiveValue::Set(Some(status));
    meal.update(&db).await.map_err(|_| ApiError::Internal)?;

    Ok(())
}
