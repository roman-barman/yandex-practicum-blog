use crate::domain::entities::User;
use crate::domain::value_objects::{Email, PasswordHash, UserName};
use secrecy::SecretString;

#[tracing::instrument(name = "Handle register user command")]
pub(crate) fn register_user_handler(cmd: RegisterUserCommand) -> Result<User, RegisterUserError> {
    let user_name = UserName::try_from(cmd.username)
        .map_err(|e| RegisterUserError::InvalidUser(e.to_string()))?;
    let email =
        Email::try_from(cmd.email).map_err(|e| RegisterUserError::InvalidUser(e.to_string()))?;
    let password_hash = PasswordHash::from(cmd.password);

    Ok(User::new(user_name, email, password_hash))
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct RegisterUserCommand {
    username: String,
    password: SecretString,
    email: String,
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RegisterUserError {
    #[error("invalid user: {0}")]
    InvalidUser(String),
}
