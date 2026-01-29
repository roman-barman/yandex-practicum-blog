use crate::api::errors::ApiError;
use crate::api::extractors::AuthenticatedUser;
use actix_web::{HttpResponse, post};

#[post("")]
#[tracing::instrument(name = "Create post")]
pub(crate) async fn create_post(user: AuthenticatedUser) -> Result<HttpResponse, ApiError> {
    tracing::info!("User: {:?}", user);
    Ok(HttpResponse::Ok().finish())
}
