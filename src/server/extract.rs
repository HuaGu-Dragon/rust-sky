use axum::{extract::FromRequestParts, http::request::Parts};

use crate::server::{error::ApiError, middleware::RawToken};

#[derive(Debug, Clone, Copy)]
pub struct Id(pub i64);

impl<S> FromRequestParts<S> for Id
where
    S: Send + Sync,
{
    #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
    #[doc = " a kind of error that can be converted into a response."]
    type Rejection = ApiError;

    #[doc = " Perform the extraction."]
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(&id) = parts.extensions.get::<i64>() {
            return Ok(Self(id));
        }
        //TODO: Handle error properly
        let token = parts
            .extensions
            .get::<RawToken>()
            .ok_or(ApiError::Unauthorized)
            .unwrap();

        let id = crate::server::auth::jwt_service().decode(&token.0)?;

        parts.extensions.insert(id);

        Ok(Self(id))
    }
}
