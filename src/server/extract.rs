use axum::{extract::FromRequestParts, http::request::Parts};

use crate::server::error::ApiError;

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
        let id = *parts
            .extensions
            .get::<i64>()
            .ok_or(ApiError::Unauthorized)
            .unwrap();

        Ok(Self(id))
    }
}
