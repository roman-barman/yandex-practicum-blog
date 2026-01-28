use crate::api::errors::ApiError;
use crate::application::auth::{VerifyUserCommand, verify_user_handler};
use crate::application::contracts::UserRepository;
use actix_web::{HttpResponse, post, web};
use std::sync::Arc;

#[post("/auth/login")]
#[tracing::instrument(name = "Login user", skip(users_repo))]
pub(crate) async fn login(
    request: web::Json<VerifyUserCommand>,
    users_repo: web::Data<Arc<dyn UserRepository>>,
) -> Result<HttpResponse, ApiError> {
    let _ = verify_user_handler(request.0, users_repo.get_ref()).await?;

    Ok(HttpResponse::Ok().finish())
}
