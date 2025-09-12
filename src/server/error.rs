use axum::{Json, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not Found")]
    NotFound,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Account is Locked")]
    AccountLocked,
    #[error("Account or Password is incorrect")]
    LoginError,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid Token: {0}")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    #[error("Internal Server Error")]
    Internal,
}

impl ApiError {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ApiError::LoginError | ApiError::InvalidToken(_) | ApiError::Unauthorized => {
                axum::http::StatusCode::UNAUTHORIZED
            }
            ApiError::Internal => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AccountLocked => axum::http::StatusCode::FORBIDDEN,
            ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: i32,
    pub msg: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = Json(ErrorResponse {
            code: 0,
            msg: self.to_string(),
        });
        (status, body).into_response()
    }
}
