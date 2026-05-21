use sea_orm::{
    ActiveValue, IntoActiveModel, QuerySelect, QueryTrait, TransactionTrait, prelude::*,
};
use sky_pojo::{
    dto::shopping_cart::CartDto,
    entities::{dish, setmeal, shopping_cart},
    vo::shopping_cart::CartVO,
};
use tracing::info;

use crate::server::error::{ApiError, ApiResult};

pub async fn add(id: i64, db: DatabaseConnection, cart_update: CartDto) -> ApiResult<()> {
    let result = shopping_cart::Entity::update_many()
        .col_expr(
            shopping_cart::Column::Number,
            Expr::col(shopping_cart::Column::Number).add(1),
        )
        .filter(shopping_cart::Column::UserId.eq(id))
        .apply_if(cart_update.dish_id, |query, dish_id| {
            query.filter(shopping_cart::Column::DishId.eq(dish_id))
        })
        .apply_if(cart_update.setmeal_id, |query, meal_id| {
            query.filter(shopping_cart::Column::SetmealId.eq(meal_id))
        })
        .exec(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    if result.rows_affected == 0 {
        let mut cart = <shopping_cart::ActiveModel as ActiveModelTrait>::default();
        cart.user_id = ActiveValue::Set(id);
        if let Some(dish) = cart_update.dish_id {
            info!("Adding dish to cart: {}", dish);
            cart.dish_id = ActiveValue::Set(Some(dish));
            let dish = dish::Entity::find_by_id(dish)
                .one(&db)
                .await
                .map_err(|_| ApiError::Internal)?
                .ok_or(ApiError::NotFound)?;
            cart.image = ActiveValue::Set(dish.image);
            cart.amount = ActiveValue::Set(dish.price.unwrap_or_default());
            cart.name = ActiveValue::Set(Some(dish.name));
            cart.dish_flavor = ActiveValue::Set(cart_update.dish_flavor);
        }
        if let Some(meal) = cart_update.setmeal_id {
            cart.setmeal_id = ActiveValue::Set(Some(meal));
            let meal = setmeal::Entity::find_by_id(meal)
                .one(&db)
                .await
                .map_err(|_| ApiError::Internal)?
                .ok_or(ApiError::NotFound)?;
            cart.image = ActiveValue::Set(meal.image);
            cart.amount = ActiveValue::Set(meal.price);
            cart.name = ActiveValue::Set(Some(meal.name));
        }
        cart.number = ActiveValue::Set(1);
        cart.insert(&db).await.map_err(|_| ApiError::Internal)?;
    }

    Ok(())
}

pub async fn list(id: i64, db: DatabaseConnection) -> ApiResult<Vec<CartVO>> {
    let carts = shopping_cart::Entity::find()
        .filter(shopping_cart::Column::UserId.eq(id))
        .all(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let carts = carts.into_iter().map(CartVO::from).collect();

    Ok(carts)
}

pub async fn clean(id: i64, db: DatabaseConnection) -> ApiResult<()> {
    shopping_cart::Entity::delete_many()
        .filter(shopping_cart::Column::UserId.eq(id))
        .exec(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    Ok(())
}

pub async fn sub(id: i64, db: DatabaseConnection, cart_update: CartDto) -> ApiResult<()> {
    let txn = db.begin().await.map_err(|_| ApiError::Internal)?;

    let carts = shopping_cart::Entity::find()
        .filter(shopping_cart::Column::UserId.eq(id))
        .apply_if(cart_update.dish_id, |query, dish_id| {
            query.filter(shopping_cart::Column::DishId.eq(dish_id))
        })
        .apply_if(cart_update.setmeal_id, |query, meal_id| {
            query.filter(shopping_cart::Column::SetmealId.eq(meal_id))
        })
        .lock(sea_orm::sea_query::LockType::Update)
        .one(&txn)
        .await
        .map_err(|_| ApiError::Internal)?;

    if let Some(cart) = carts {
        if cart.number == 1 {
            cart.into_active_model()
                .delete(&txn)
                .await
                .map_err(|_| ApiError::Internal)?;
        } else {
            let mut cart = cart.into_active_model();

            cart.number = ActiveValue::Set(cart.number.unwrap() + 1);
            cart.update(&txn).await.map_err(|_| ApiError::Internal)?;
        }
    }

    txn.commit().await.map_err(|_| ApiError::Internal)?;

    // NOTE: If no cart is found, do nothing
    Ok(())
}
