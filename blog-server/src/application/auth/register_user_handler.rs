use crate::application::auth::password::calculate_password_hash;
use crate::application::contracts::UserRepository;
use crate::domain::entities::User;
use crate::domain::value_objects::{
    Email, EmailError, Password, PasswordError, UserName, UserNameError,
};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use secrecy::SecretString;
use std::sync::Arc;
use tokio::task::JoinError;

#[tracing::instrument(name = "Handle register user command", skip(users_repo))]
pub(crate) async fn register_user_handler(
    cmd: RegisterUserCommand,
    users_repo: &Arc<dyn UserRepository>,
) -> Result<User, RegisterUserError> {
    let user_name = UserName::try_from(cmd.username)?;
    let email = Email::try_from(cmd.email)?;
    let password = Password::try_from(cmd.password)?;
    let salt = SaltString::generate(&mut OsRng);
    let password_hash =
        tokio::task::spawn_blocking(move || calculate_password_hash(&password, &salt))
            .await?
            .map_err(|err| RegisterUserError::Unexpected(err.to_string()))?;

    let is_username_or_email_exists = users_repo
        .exist(&user_name, &email)
        .await
        .map_err(|err| RegisterUserError::Unexpected(err.to_string()))?;

    if is_username_or_email_exists {
        return Err(RegisterUserError::UsernameOrEmailExist);
    }

    let user = User::new(user_name, email, password_hash);
    users_repo
        .create(&user)
        .await
        .map_err(|err| RegisterUserError::Unexpected(err.to_string()))?;

    Ok(user)
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct RegisterUserCommand {
    username: String,
    password: SecretString,
    email: String,
}

impl RegisterUserCommand {
    pub(crate) fn new(username: String, password: SecretString, email: String) -> Self {
        Self {
            username,
            password,
            email,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RegisterUserError {
    #[error("invalid user: {0}")]
    InvalidUser(String),
    #[error("username or email already exists")]
    UsernameOrEmailExist,
    #[error("unexpected error: {0}")]
    Unexpected(String),
}

impl From<UserNameError> for RegisterUserError {
    fn from(err: UserNameError) -> Self {
        RegisterUserError::InvalidUser(err.to_string())
    }
}

impl From<EmailError> for RegisterUserError {
    fn from(err: EmailError) -> Self {
        match err {
            EmailError::Regex(err) => RegisterUserError::Unexpected(err.to_string()),
            _ => RegisterUserError::InvalidUser(err.to_string()),
        }
    }
}

impl From<PasswordError> for RegisterUserError {
    fn from(value: PasswordError) -> Self {
        RegisterUserError::InvalidUser(value.to_string())
    }
}

impl From<JoinError> for RegisterUserError {
    fn from(value: JoinError) -> Self {
        RegisterUserError::Unexpected(value.to_string())
    }
}
