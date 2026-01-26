use crate::domain::value_objects::{Password, PasswordHash};
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, PasswordVerifier, Version};
use secrecy::SecretString;

pub(super) fn calculate_password_hash(
    password: &Password,
    salt: &SaltString,
) -> anyhow::Result<PasswordHash> {
    let argon2 = create_argon2()?;
    let password_hash = argon2
        .hash_password(password.as_ref(), salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()).context("failed hash password"))?
        .to_string();
    Ok(SecretString::from(password_hash).into())
}

pub(super) fn verify_password(hash: &PasswordHash, password: &Password) -> anyhow::Result<bool> {
    let argon2 = create_argon2()?;
    let hash: argon2::password_hash::PasswordHash<'_> = hash.try_into()?;
    let result = argon2.verify_password(password.as_ref(), &hash);
    match result {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::Password) => Ok(false),
        Err(e) => Err(anyhow::anyhow!(e.to_string()).context("failed verify password")),
    }
}

fn create_argon2() -> anyhow::Result<Argon2<'static>> {
    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).map_err(|e| {
            anyhow::anyhow!(e.to_string()).context("failed to create argon2 params")
        })?,
    );
    Ok(argon2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::password_hash::rand_core::OsRng;

    #[test]
    fn test_verify_password() {
        let password = Password::try_from(SecretString::from("Password123!")).unwrap();
        let salt = SaltString::generate(&mut OsRng);
        let hash = calculate_password_hash(&password, &salt).unwrap();

        assert!(verify_password(&hash, &password).unwrap());
        assert!(
            !verify_password(
                &hash,
                &Password::try_from(SecretString::from("wrong_Password123!")).unwrap()
            )
            .unwrap()
        );
    }
}
