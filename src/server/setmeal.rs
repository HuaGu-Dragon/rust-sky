use futures_util::{future::try_join_all, try_join};
use sea_orm::{ActiveValue, Condition, IntoActiveModel, QueryTrait, TransactionTrait, prelude::*};
use sky_pojo::{
    dto::setmeal::{SetmealDto, SetmealPageQuery},
    entities::{category, dish, setmeal, setmeal_dish},
    vo::{
        Page,
        setmeal::{SetmealDetailVo, SetmealVo, UserSetmealDishVo},
    },
};

use crate::{
    server::{
        ENABLE,
        error::{ApiError, ApiResult},
    },
    update_params,
};

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

pub async fn update(db: DatabaseConnection, setmeal: SetmealDto) -> ApiResult<()> {
    let meal_has = setmeal::Entity::find_by_id(
        setmeal
            .id
            .ok_or_else(|| ApiError::BadRequest("ID is required".to_string()))?,
    )
    .one(&db)
    .await
    .map_err(|_| ApiError::Internal)?
    .ok_or(ApiError::NotFound)?;

    let id = meal_has.id;
    let mut meal_has = meal_has.into_active_model();

    update_params!(meal_has, name, setmeal.name);
    update_params!(meal_has, category_id, setmeal.category_id);
    update_params!(meal_has, price, setmeal.price);
    update_params!(meal_has, image, Some(setmeal.image));
    update_params!(meal_has, description, setmeal.description);
    update_params!(meal_has, status, Some(setmeal.status));

    let txn = db.begin().await.map_err(|_| ApiError::Internal)?;

    setmeal_dish::Entity::delete_many()
        .filter(setmeal_dish::Column::SetmealId.eq(id))
        .exec(&txn)
        .await
        .map_err(|_| ApiError::Internal)?;

    let task = setmeal.setmeal_dishes.into_iter().map(|m| {
        let mut model = m.into_active_model();
        model.setmeal_id = ActiveValue::Set(Some(id));
        model.insert(&txn)
    });

    try_join_all(task).await.map_err(|_| ApiError::Internal)?;
    meal_has
        .update(&txn)
        .await
        .map_err(|_| ApiError::Internal)?;

    txn.commit().await.map_err(|_| ApiError::Internal)?;
    Ok(())
}

pub async fn list(db: DatabaseConnection, category_id: i64) -> ApiResult<Vec<SetmealVo>> {
    let meals = setmeal::Entity::find()
        .filter(
            Condition::all()
                .add(setmeal::Column::CategoryId.eq(category_id))
                .add(setmeal::Column::Status.eq(ENABLE)),
        )
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let meals = meals.into_iter().map(|meal| meal.into()).collect();
    Ok(meals)
}

pub async fn get_dish(db: DatabaseConnection, id: i64) -> ApiResult<Vec<UserSetmealDishVo>> {
    let meal_dishes = setmeal_dish::Entity::find()
        .filter(setmeal_dish::Column::SetmealId.eq(id))
        .find_also_related(dish::Entity)
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let dishes = meal_dishes
        .into_iter()
        .map(UserSetmealDishVo::from)
        .collect();

    Ok(dishes)
}
