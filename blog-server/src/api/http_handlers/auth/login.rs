use crate::api::errors::ApiError;
use actix_web::{HttpResponse, post};

#[post("/auth/login")]
#[tracing::instrument(name = "Login user")]
pub(crate) async fn login() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
}
