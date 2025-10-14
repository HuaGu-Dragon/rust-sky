use std::path::PathBuf;

use axum::{
    Router,
    extract::{Multipart, multipart::Field},
    routing::post,
};
use chrono::{Datelike, NaiveDateTime, Utc};
use sha1::{Digest, Sha1};
use tokio::io::AsyncWriteExt;
use tracing::error;

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
    let mut path = String::from("/");
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

        let mut hasher = Sha1::new();
        hasher.update(file_name);
        let file_name = format!(
            "{}-{}",
            Utc::now().timestamp(),
            hex::encode(hasher.finalize())
        );
        let file_path = format!("{}{}", dir, file_name);

        save_file(field, &file_path).await?;

        path.push_str(&file_path);
    }
    Ok(ApiResponse::success(path))
}

async fn get_dir(date: NaiveDateTime) -> ApiResult<String> {
    let dir = format!(
        "./upload/{}/{:02}/{:02}/",
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

async fn save_file(mut field: Field<'_>, file_path: &str) -> ApiResult<()> {
    let mut file = match tokio::fs::File::create(file_path).await {
        Ok(file) => file,
        Err(e) => {
            error!("Create file error: {:?}", e);
            return Err(ApiError::Internal);
        }
    };

    while let Some(chunk) = field.chunk().await.map_err(|_| ApiError::Internal)? {
        file.write_all(chunk.as_ref())
            .await
            .map_err(|_| ApiError::Internal)?;
    }

    Ok(())
}
