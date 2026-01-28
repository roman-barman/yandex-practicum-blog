use crate::application::auth::password::verify_password;
use crate::application::contracts::UserRepository;
use crate::domain::entities::User;
use crate::domain::value_objects::{Password, PasswordError, UserName, UserNameError};
use secrecy::SecretString;
use std::sync::Arc;

pub(crate) async fn verify_user_handler(
    cmd: VerifyUserCommand,
    users_repo: &Arc<dyn UserRepository>,
) -> Result<User, VerifyUserError> {
    let user_name = UserName::try_from(cmd.username)?;
    let password = Password::try_from(cmd.password)?;
    let user = users_repo
        .get(&user_name)
        .await
        .map_err(|err| VerifyUserError::Unexpected(err.to_string()))?
        .ok_or(VerifyUserError::UserNotFound)?;

    let is_verified = verify_password(user.password_hash(), &password)
        .map_err(|err| VerifyUserError::Unexpected(err.to_string()))?;

    if is_verified {
        Ok(user)
    } else {
        Err(VerifyUserError::InvalidUserNameOrPassword(
            "password is incorrect".to_string(),
        ))
    }
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct VerifyUserCommand {
    username: String,
    password: SecretString,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum VerifyUserError {
    #[error("invalid user username or password: {0}")]
    InvalidUserNameOrPassword(String),
    #[error("user not found")]
    UserNotFound,
    #[error("unexpected error: {0}")]
    Unexpected(String),
}

impl From<UserNameError> for VerifyUserError {
    fn from(value: UserNameError) -> Self {
        VerifyUserError::InvalidUserNameOrPassword(value.to_string())
    }
}

impl From<PasswordError> for VerifyUserError {
    fn from(value: PasswordError) -> Self {
        VerifyUserError::InvalidUserNameOrPassword(value.to_string())
    }
}
