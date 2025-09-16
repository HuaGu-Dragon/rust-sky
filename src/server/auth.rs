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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum JwtAuthKey {
    UserId,
    AdminId,
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
    pub fn encode(&self, key: JwtAuthKey, id: i64) -> ApiResult<String> {
        let now = get_current_timestamp();

        let key = key as u8;
        let sub = format!("{key}:{id}");
        let claims = Claims {
            sub,
            exp: now.saturating_add(self.expiration),
            iat: now,
        };

        Ok(jsonwebtoken::encode(&self.header, &claims, &self.encode_key).unwrap())
    }

    //TODO: Handle errors properly
    //TODO: Return custom error when token is expired or invalid
    pub fn decode(&self, token: &str) -> ApiResult<(JwtAuthKey, i64)> {
        let token_data =
            jsonwebtoken::decode::<Claims>(token, &self.decode_key, &self.validation)?.claims;

        let (key, id) = token_data.sub.split_once(':').ok_or_else(|| {
            jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
        })?;
        let key = key.parse::<u8>().map_err(|_| {
            jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
        })?;

        let key = match key {
            0 => JwtAuthKey::UserId,
            1 => JwtAuthKey::AdminId,
            _ => {
                return Err(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                )
                .into());
            }
        };
        let id = id.parse::<i64>().map_err(|_| {
            jsonwebtoken::errors::Error::from(jsonwebtoken::errors::ErrorKind::InvalidToken)
        })?;
        Ok((key, id))
    }
}

pub fn jwt_service() -> &'static JwtService {
    &JWT_SERVICE
}
