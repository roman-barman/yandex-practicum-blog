use crate::api::errors::ApiError;
use crate::application::auth::{RegisterUserCommand, register_user_handler};
use crate::application::contracts::UserRepository;
use actix_web::{HttpResponse, post, web};
use std::sync::Arc;

#[post("/auth/register")]
#[tracing::instrument(name = "Register a new user", skip(users_repo))]
pub(crate) async fn register_user(
    request: web::Json<RegisterUserCommand>,
    users_repo: web::Data<Arc<dyn UserRepository>>,
) -> Result<HttpResponse, ApiError> {
    let _ = register_user_handler(request.0, users_repo.get_ref()).await?;
    Ok(HttpResponse::Created().finish())
}
