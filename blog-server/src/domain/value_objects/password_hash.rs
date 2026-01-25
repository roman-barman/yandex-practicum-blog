use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, PartialEq)]
pub struct PasswordHash(String);

impl From<SecretString> for PasswordHash {
    fn from(s: SecretString) -> Self {
        Self(s.expose_secret().to_string())
    }
}
