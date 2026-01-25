use crate::domain::entities::User;
use crate::domain::value_objects::{Email, EmailError, PasswordHash, UserName, UserNameError};
use secrecy::SecretString;

#[tracing::instrument(name = "Handle register user command")]
pub(crate) fn register_user_handler(cmd: RegisterUserCommand) -> Result<User, RegisterUserError> {
    let user_name = UserName::try_from(cmd.username)?;
    let email = Email::try_from(cmd.email)?;
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
