use crate::application::auth::{RegisterUserError, VerifyUserError};
use crate::application::blog::CreatePostError;
use actix_web::ResponseError;
use actix_web::http::header::ContentType;
use serde::Serialize;
use tracing_log::log::error;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ApiError {
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("{0}")]
    Conflict(String),
    #[error("internal server error")]
    InternalServerError(String),
    #[error("unauthorized")]
    Unauthorized(String),
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::UnprocessableEntity(_) => actix_web::http::StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::InternalServerError(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::Conflict(_) => actix_web::http::StatusCode::CONFLICT,
            ApiError::Unauthorized(_) => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        error!("Error: {:?}", self);
        create_error_response(self.status_code(), self.to_string())
    }
}

pub(crate) fn create_error_response(
    status_code: actix_web::http::StatusCode,
    error: String,
) -> actix_web::HttpResponse {
    actix_web::HttpResponse::build(status_code)
        .insert_header(ContentType::json())
        .json(ErrorResponse { error })
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<RegisterUserError> for ApiError {
    fn from(err: RegisterUserError) -> Self {
        match err {
            RegisterUserError::InvalidUser(error) => ApiError::UnprocessableEntity(error),
            RegisterUserError::Unexpected(error) => ApiError::InternalServerError(error),
            RegisterUserError::UsernameOrEmailExist => ApiError::Conflict(err.to_string()),
        }
    }
}

impl From<VerifyUserError> for ApiError {
    fn from(err: VerifyUserError) -> Self {
        match err {
            VerifyUserError::InvalidUserNameOrPassword(error) => ApiError::Unauthorized(error),
            VerifyUserError::UserNotFound => ApiError::Unauthorized(err.to_string()),
            VerifyUserError::Unexpected(error) => ApiError::InternalServerError(error),
        }
    }
}

impl From<CreatePostError> for ApiError {
    fn from(err: CreatePostError) -> Self {
        match err {
            CreatePostError::InvalidTitle(error) => {
                ApiError::UnprocessableEntity(error.to_string())
            }
            CreatePostError::Unexpected(error) => ApiError::InternalServerError(error),
        }
    }
}
