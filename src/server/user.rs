use sea_orm::{ActiveValue, prelude::*};
use sky_pojo::entities::user;

use crate::{
    config,
    server::error::{ApiError, ApiResult},
};

async fn get_openid(code: String) -> ApiResult<String> {
    let config = config::get().app();
    let mut url = reqwest::Url::parse("https://api.weixin.qq.com/sns/jscode2session")
        .map_err(|_| ApiError::Internal)?;
    url.query_pairs_mut()
        .append_pair("appid", config.app_id())
        .append_pair("secret", config.app_secret())
        .append_pair("js_code", &code)
        .append_pair("grant_type", "authorization_code");

    let resp = reqwest::get(url).await.map_err(|_| ApiError::Internal)?;

    let resp: serde_json::Value = resp.json().await.map_err(|_| ApiError::Internal)?;
    if resp.get("errcode").is_some() {
        return Err(ApiError::Unauthorized);
    }

    let openid = resp
        .get("openid")
        .and_then(|v| v.as_str())
        .ok_or(ApiError::Unauthorized)?;

    Ok(openid.to_string())
}

pub async fn login(db: DatabaseConnection, code: String) -> ApiResult<user::Model> {
    let openid = get_openid(code).await?;

    let user = user::Entity::find()
        .filter(user::Column::Openid.eq(&openid))
        .one(&db)
        .await
        .map_err(|_| ApiError::Internal)?;

    let user = match user {
        Some(existing_user) => existing_user,
        None => {
            let new_user = user::ActiveModel {
                id: ActiveValue::NotSet,
                openid: ActiveValue::Set(Some(openid)),
                ..Default::default()
            };

            new_user.insert(&db).await.map_err(|_| ApiError::Internal)?
        }
    };

    Ok(user)
}
