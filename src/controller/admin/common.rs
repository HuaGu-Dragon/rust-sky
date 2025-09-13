use axum::{Router, extract::Multipart, routing::post};

use crate::{
    app::AppState,
    server::{ApiReturn, response::ApiResponse},
};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/upload", post(upload))
}

// Upload file
//TODO: Save file to local or cloud storage and return the file path
// I don't actually know which way to go, I don't have a oss account, but I can save it to local first
async fn upload(mut _multiple: Multipart) -> ApiReturn<String> {
    // while let Ok(Some(_field)) = multiple.next_field().await {}
    // todo!()
    Ok(ApiResponse::success(
        "https://avatars.githubusercontent.com/u/178029962?v=4".to_string(),
    ))
}
