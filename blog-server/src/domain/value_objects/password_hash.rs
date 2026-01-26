use secrecy::{ExposeSecret, SecretString};

#[derive(Debug)]
pub struct PasswordHash(SecretString);

impl From<SecretString> for PasswordHash {
    fn from(s: SecretString) -> Self {
        Self(s)
    }
}

impl PartialEq for PasswordHash {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}
