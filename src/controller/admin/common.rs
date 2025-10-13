use std::path::PathBuf;

use axum::{Router, extract::Multipart, routing::post};
use chrono::{Datelike, NaiveDateTime};
use tracing::{error, info};

use crate::{
    app::AppState,
    server::{
        ApiReturn,
        error::{ApiError, ApiResult},
        response::ApiResponse,
    },
};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/upload", post(upload))
}

// Upload file
//TODO: Save file to local or cloud storage and return the file path
// I don't actually know which way to go, I don't have a oss account, but I can save it to local first
async fn upload(mut multiple: Multipart) -> ApiReturn<String> {
    // while let Ok(Some(_field)) = multiple.next_field().await {}
    // todo!()
    while let Some(field) = multiple
        .next_field()
        .await
        .map_err(|_| ApiError::Internal)?
    {
        let file_name = field.file_name().unwrap_or("null");

        let dir = match get_dir(chrono::Utc::now().naive_utc()).await {
            Ok(dir) => dir,
            Err(e) => {
                error!("Create upload dir error: {:?}", e);
                return Err(ApiError::Internal);
            }
        };

        let file_path = format!("{}/{}", dir, file_name);

        info!("Upload file: {}", file_path);
    }
    Ok(ApiResponse::success(
        "https://avatars.githubusercontent.com/u/178029962?v=4".to_string(),
    ))
}

async fn get_dir(date: NaiveDateTime) -> ApiResult<String> {
    let dir = format!(
        "/upload/{}/{:02}/{:02}/",
        date.year(),
        date.month(),
        date.day()
    );
    let dir_path = PathBuf::from(&dir);

    if !dir_path.exists() {
        tokio::fs::create_dir_all(dir_path)
            .await
            .map_err(|_| ApiError::Internal)?;
    }

    Ok(dir)
}
