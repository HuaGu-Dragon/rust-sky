use std::sync::LazyLock;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};

use crate::{config, server::error::ApiResult};

static JWT_SERVICE: LazyLock<JwtService> = LazyLock::new(JwtService::new);

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64,
    iat: u64,
}

pub struct JwtService {
    encode_key: EncodingKey,
    decode_key: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: u64,
}

impl JwtService {
    pub fn new() -> Self {
        let config = config::get().auth();
        let mut validation = Validation::new(config.algorithm());
        validation.set_required_spec_claims(&["sub", "exp", "iat"]);

        Self {
            encode_key: EncodingKey::from_secret(config.secret().as_bytes()),
            decode_key: DecodingKey::from_secret(config.secret().as_bytes()),
            header: Header::new(config.algorithm()),
            validation,
            expiration: config.expiration(),
        }
    }

    //TODO: Handle errors properly
    pub fn encode(&self, id: i64) -> ApiResult<String> {
        let now = get_current_timestamp();

        let claims = Claims {
            sub: id.to_string(),
            exp: now.saturating_add(self.expiration),
            iat: now,
        };

        Ok(jsonwebtoken::encode(&self.header, &claims, &self.encode_key).unwrap())
    }

    //TODO: Handle errors properly
    //TODO: Return custom error when token is expired or invalid
    pub fn decode(&self, token: &str) -> ApiResult<i64> {
        let token_data = jsonwebtoken::decode::<Claims>(token, &self.decode_key, &self.validation)
            .unwrap()
            .claims;
        Ok(token_data.sub.parse::<i64>().unwrap())
    }
}

pub fn jwt_service() -> &'static JwtService {
    &JWT_SERVICE
}
