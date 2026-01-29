use crate::api::errors::ApiError;
use actix_web::{HttpResponse, post};

#[post("")]
#[tracing::instrument(name = "Create post")]
pub(crate) async fn create_post() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().finish())
}
