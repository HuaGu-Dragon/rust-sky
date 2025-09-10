use axum::{Json, response::IntoResponse};
use serde::Serialize;

pub type ApiResult<T> = Result<T, ApiError>;
pub enum ApiError {
    Internal,
}

impl ApiError {
    fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ApiError::Internal => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
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
            msg: "Internal Server Error".to_string(),
        });
        (status, body).into_response()
    }
}
