use crate::domain::value_objects::{Password, PasswordHash};
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use secrecy::SecretString;

pub(super) fn calculate_password_hash(
    password: &Password,
    salt: &SaltString,
) -> anyhow::Result<PasswordHash> {
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).map_err(|e| {
            anyhow::anyhow!(e.to_string()).context("failed to create argon2 params")
        })?,
    );
    let password_hash = argon2
        .hash_password(password.as_ref(), salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()).context("failed hash password"))?
        .to_string();
    Ok(SecretString::from(password_hash).into())
}
