use crate::application::auth::RegisterUserError;
use actix_web::ResponseError;
use actix_web::http::header::ContentType;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ApiError {
    #[error("{0}")]
    UnprocessableEntity(String),
    #[error("internal server error")]
    InternalServerError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            ApiError::UnprocessableEntity(_) => actix_web::http::StatusCode::UNPROCESSABLE_ENTITY,
            ApiError::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorResponse {
                error: self.to_string(),
            })
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<RegisterUserError> for ApiError {
    fn from(err: RegisterUserError) -> Self {
        match err {
            RegisterUserError::InvalidUser(error) => ApiError::UnprocessableEntity(error),
            RegisterUserError::Unexpected(_) => ApiError::InternalServerError,
        }
    }
}
