use crate::api::errors::ApiError;
use crate::application::auth::{RegisterUserCommand, register_user_handler};
use actix_web::{HttpResponse, post, web};

#[post("/auth/register")]
#[tracing::instrument(name = "Register a new user")]
pub(crate) async fn register_user(
    request: web::Json<RegisterUserCommand>,
) -> Result<HttpResponse, ApiError> {
    let _ = register_user_handler(request.0).await?;
    Ok(HttpResponse::Created().finish())
}
