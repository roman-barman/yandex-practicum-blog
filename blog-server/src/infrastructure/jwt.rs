use crate::domain::entities::User;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

const TOKEN_EXPIRATION_TIME: usize = 60 * 60;

pub(crate) struct JwtService {
    secret: SecretString,
}

impl JwtService {
    pub(crate) fn new(secret: SecretString) -> Self {
        Self { secret }
    }

    #[tracing::instrument(name = "Generate JWT", skip(self))]
    pub(crate) fn generate_jwt(&self, user: &User) -> anyhow::Result<String> {
        let claims = Claims {
            sub: user.id().as_ref().clone(),
            username: user.username().as_ref().to_string(),
            exp: SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() as usize
                + TOKEN_EXPIRATION_TIME,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.expose_secret().as_bytes()),
        )?;
        Ok(token)
    }

    #[tracing::instrument(name = "Decode JWT", skip(self, token))]
    pub(crate) fn decode_jwt(&self, token: &str) -> Result<Claims, JwtDecodeError> {
        let token_data: TokenData<Claims> = decode(
            token,
            &DecodingKey::from_secret(self.secret.expose_secret().as_bytes()),
            &Validation::default(),
        )?;
        Ok(token_data.claims)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Claims {
    sub: Uuid,
    username: String,
    exp: usize,
}

impl Claims {
    pub(crate) fn sub(&self) -> Uuid {
        self.sub
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum JwtDecodeError {
    #[error("unauthorized")]
    Unauthorized(#[from] jsonwebtoken::errors::Error),
}
