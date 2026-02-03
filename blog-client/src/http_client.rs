use crate::RegisterUserCommand;
use crate::errors::RegisterUserError;
use serde::Deserialize;

pub(crate) async fn register_user(
    address: &str,
    cmd: RegisterUserCommand,
) -> Result<(), RegisterUserError> {
    let request = serde_json::json!({
        "username": cmd.get_username(),
        "password": cmd.get_password(),
        "email": cmd.get_email(),
    });

    let response = reqwest::Client::new()
        .post(&format!("{}/api/auth/register", address))
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        reqwest::StatusCode::CONFLICT => Err(RegisterUserError::UsernameOrEmailExist),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY => Err(RegisterUserError::InvalidUser(
            response.json::<ErrorResponse>().await?.error,
        )),
        _ => Err(RegisterUserError::Unexpected(
            response.json::<ErrorResponse>().await?.error,
        )),
    }
}

impl From<reqwest::Error> for RegisterUserError {
    fn from(err: reqwest::Error) -> Self {
        RegisterUserError::Unexpected(err.to_string())
    }
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}
