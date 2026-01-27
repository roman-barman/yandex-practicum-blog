use secrecy::{ExposeSecret, SecretString};

#[derive(Debug)]
pub struct PasswordHash(SecretString);

impl AsRef<SecretString> for PasswordHash {
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

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

impl<'a> TryFrom<&'a PasswordHash> for argon2::password_hash::PasswordHash<'a> {
    type Error = argon2::password_hash::Error;
    fn try_from(value: &'a PasswordHash) -> Result<Self, Self::Error> {
        match argon2::password_hash::PasswordHash::new(value.0.expose_secret()) {
            Ok(hash) => Ok(hash),
            Err(e) => Err(e),
        }
    }
}
