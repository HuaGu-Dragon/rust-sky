use futures_util::{future::try_join_all, try_join};
use sea_orm::{ActiveValue, Condition, IntoActiveModel, QueryTrait, TransactionTrait, prelude::*};
use sky_pojo::{
    dto::dish::{DishDto, DishQueryDto},
    entities::{
        category,
        dish::{self},
        dish_flavor, setmeal_dish,
    },
    vo::{
        Page,
        dish::{DishDetailVO, DishVO},
    },
};

use crate::{
    server::{
        DISABLE, ENABLE,
        error::{ApiError, ApiResult},
    },
    update_params,
};

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
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let (category, flavors) = try_join!(
        dish.find_related(category::Entity).one(&db),
        dish.find_related(dish_flavor::Entity).all(&db)
    )
    .map_err(|_| ApiError::Internal)?;

    let dish = (dish, category).into();
    let dish = (dish, flavors).into();

    Ok(dish)
}

pub async fn update(user_id: i64, db: DatabaseConnection, dish: DishDto) -> ApiResult<()> {
    let dish_has = dish::Entity::find_by_id(dish.id)
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?
        .ok_or(ApiError::NotFound)?;

    let id = dish_has.id;
    let mut dish_has = dish_has.into_active_model();

    update_params!(dish_has, name, dish.name);
    update_params!(dish_has, category_id, dish.category_id);
    update_params!(dish_has, price, Some(dish.price));
    update_params!(dish_has, image, Some(dish.image));
    update_params!(dish_has, description, dish.description);
    update_params!(dish_has, status, dish.status);
    update_params!(dish_has, update_user, Some(user_id));

    let txn = db.begin().await.map_err(|_| ApiError::Internal)?;

    dish_flavor::Entity::delete_many()
        .filter(dish_flavor::Column::DishId.eq(dish.id))
        .exec(&txn)
        .await
        .map_err(|_| ApiError::Internal)?;

    let tasks = dish.flavors.unwrap_or_default().into_iter().map(|f| {
        let mut model = f.into_active_model();
        model.id = ActiveValue::NotSet;
        model.dish_id = ActiveValue::Set(id);
        model.insert(&txn)
    });

    try_join_all(tasks).await.map_err(|_| ApiError::Internal)?;

    dish_has
        .update(&txn)
        .await
        .map_err(|_| ApiError::Internal)?;

    txn.commit().await.map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn delete(db: DatabaseConnection, id: Vec<i64>) -> ApiResult<()> {
    let valid_dishes = dish::Entity::find()
        .filter(
            Condition::all()
                .add(dish::Column::Id.is_in(id))
                .add(dish::Column::Status.eq(DISABLE)),
        )
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    if valid_dishes.is_empty() {
        return Err(ApiError::BadRequest(
            "Cannot delete dishes that are currently being sold".to_string(),
        ));
    }

    let valid_ids: Vec<i64> = valid_dishes.iter().map(|d| d.id).collect();

    let in_meals = setmeal_dish::Entity::find()
        .filter(setmeal_dish::Column::DishId.is_in(valid_ids.clone()))
        .count(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    if in_meals > 0 {
        return Err(ApiError::BadRequest(
            "Cannot delete dishes that are part of a set meal".to_string(),
        ));
    }

    let txn = db.begin().await.map_err(|_| ApiError::Internal)?;

    dish_flavor::Entity::delete_many()
        .filter(dish_flavor::Column::DishId.is_in(valid_ids.clone()))
        .exec(&txn)
        .await
        .map_err(|_| ApiError::Internal)?;

    dish::Entity::delete_many()
        .filter(dish::Column::Id.is_in(valid_ids))
        .exec(&txn)
        .await
        .map_err(|_| ApiError::Internal)?;

    txn.commit().await.map_err(|_| ApiError::Internal)?;

    Ok(())
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

pub async fn list(db: DatabaseConnection, category_id: i64) -> ApiResult<Vec<DishVO>> {
    let dishes = dish::Entity::find()
        .filter(
            Condition::all()
                .add(dish::Column::CategoryId.eq(category_id))
                .add(dish::Column::Status.eq(ENABLE)),
        )
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let dishes = dishes.into_iter().map(DishVO::from).collect();

    Ok(dishes)
}
