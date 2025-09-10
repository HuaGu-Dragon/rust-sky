use std::pin::Pin;

use axum::{RequestExt, body::Body, extract::Request, response::Response};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use sea_orm::sea_query::token;
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::info;

use crate::server::auth;

#[derive(Clone, Copy)]
pub struct AuthLayer;

impl AsyncAuthorizeRequest<Body> for AuthLayer {
    type RequestBody = Body;

    type ResponseBody = Body;

    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send
                + 'static,
        >,
    >;

    fn authorize(&mut self, mut request: axum::http::Request<Body>) -> Self::Future {
        Box::pin(async move {
            let auth_header = request
                .extract_parts::<TypedHeader<Authorization<Bearer>>>()
                .await;

            if let Ok(TypedHeader(Authorization(bearer))) = auth_header {
                if let Ok(id) = auth::jwt_service().decode(bearer.token()) {
                    request.extensions_mut().insert(id);
                }
            } else if let Some(token) = request.headers().get("token") {
                if let Ok(token_str) = token.to_str() {
                    if let Ok(id) = auth::jwt_service().decode(token_str) {
                        request.extensions_mut().insert(id);
                    }
                }
            }

            Ok(request)
        })
    }
}
