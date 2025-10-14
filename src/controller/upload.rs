use std::path::PathBuf;

use axum::{
    Router,
    body::Body,
    extract::Path,
    response::{IntoResponse, Response},
    routing::get,
};
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_TYPE};

use crate::{app::AppState, server::error::ApiError};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/{*file}", get(server_file))
}

pub async fn server_file(Path(file): Path<String>) -> Response<Body> {
    let path = PathBuf::from(format!("./upload/{}", file));
    if !path.exists() || !path.starts_with("./upload") {
        return ApiError::Internal.into_response();
    }

    match tokio::fs::read(&path).await {
        Ok(file) => {
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            Response::builder()
                .header(CONTENT_TYPE, "application/octet-stream")
                .header(
                    CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", file_name),
                )
                .body(file.into())
                .unwrap()
        }
        Err(_) => ApiError::Internal.into_response(),
    }
}
