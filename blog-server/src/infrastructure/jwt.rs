use crate::domain::entities::User;
use jsonwebtoken::{EncodingKey, Header, encode};
use secrecy::{ExposeSecret, SecretString};
use serde::Serialize;
use std::time::SystemTime;

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
            sub: user.id().as_ref().to_string(),
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
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    username: String,
    exp: usize,
}
