use axum::{extract::FromRequestParts, http::request::Parts};

use crate::server::{auth::JwtAuthKey, error::ApiError};

#[derive(Debug, Clone, Copy)]
pub struct AdminId(pub i64);

impl<S> FromRequestParts<S> for AdminId
where
    S: Send + Sync,
{
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    type Rejection = ApiError;

    #[doc = " Perform the extraction."]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        //TODO: Handle error properly
        let (key, id) = parts
            .extensions
            .get::<(JwtAuthKey, i64)>()
            .ok_or(ApiError::Unauthorized)?;

        if *key != JwtAuthKey::AdminId {
            return Err(ApiError::Forbidden);
        }

        Ok(Self(*id))
    }
}
