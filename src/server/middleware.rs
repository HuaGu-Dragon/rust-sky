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
                match auth::jwt_service().decode(bearer.token()) {
                    Ok(token) => {
                        request.extensions_mut().insert(token);
                    }
                    Err(e) => {
                        request.extensions_mut().insert(e.to_string());
                    }
                }
            } else if let Some(token) = request.headers().get("token")
                && let Ok(token_str) = token.to_str()
            {
                match auth::jwt_service().decode(token_str) {
                    Ok(token) => {
                        request.extensions_mut().insert(token);
                    }
                    Err(e) => {
                        request.extensions_mut().insert(e.to_string());
                    }
                }
            }

            Ok(request)
        })
    }
}
