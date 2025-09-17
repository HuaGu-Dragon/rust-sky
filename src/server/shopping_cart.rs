use sea_orm::{ActiveValue, IntoActiveModel, QueryTrait, prelude::*};
use sky_pojo::{
    dto::shopping_cart::CartDto,
    entities::{dish, setmeal, shopping_cart},
};
use tracing::info;

use crate::server::error::{ApiError, ApiResult};

pub async fn add(id: i64, db: DatabaseConnection, cart_update: CartDto) -> ApiResult<()> {
    let carts = shopping_cart::Entity::find()
        .filter(shopping_cart::Column::UserId.eq(id))
        .apply_if(cart_update.dish_id, |query, dish_id| {
            query.filter(shopping_cart::Column::DishId.eq(dish_id))
        })
        .apply_if(cart_update.setmeal_id, |query, meal_id| {
            query.filter(shopping_cart::Column::SetmealId.eq(meal_id))
        })
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    if let Some(mut cart) = carts {
        cart.number = cart.number.saturating_add(1);
        let cart = cart.into_active_model();
        cart.update(&db).await.map_err(|_| ApiError::Internal)?;
        return Ok(());
    }

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

    Ok(())
}
