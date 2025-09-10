use std::pin::Pin;

use axum::{RequestExt, body::Body, extract::Request, response::Response};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tower_http::auth::AsyncAuthorizeRequest;

use crate::server::auth;

#[derive(Clone)]
pub struct AuthLayer;

#[derive(Clone)]
pub struct RawToken(pub String);

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
                request
                    .extensions_mut()
                    .insert(RawToken(bearer.token().to_string()));
            } else if let Some(token) = request.headers().get("token") {
                if let Ok(token_str) = token.to_str() {
                    let token_str = token_str.to_string();
                    request.extensions_mut().insert(RawToken(token_str));
                }
            }

            Ok(request)
        })
    }
}
