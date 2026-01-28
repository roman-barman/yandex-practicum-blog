use crate::api::errors::ApiError;
use crate::application::auth::{VerifyUserCommand, verify_user_handler};
use crate::application::contracts::UserRepository;
use crate::infrastructure::JwtService;
use actix_web::{HttpResponse, post, web};
use std::sync::Arc;
use uuid::Uuid;

#[post("/auth/login")]
#[tracing::instrument(name = "Login user", skip(users_repo, jwt_service))]
pub(crate) async fn login(
    request: web::Json<VerifyUserCommand>,
    users_repo: web::Data<Arc<dyn UserRepository>>,
    jwt_service: web::Data<JwtService>,
) -> Result<HttpResponse, ApiError> {
    let user = verify_user_handler(request.0, users_repo.get_ref()).await?;
    let token = jwt_service
        .generate_jwt(&user)
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let response = Response {
        token: &token,
        user: UserResponse {
            id: user.id().as_ref(),
            username: user.username().as_ref(),
            email: user.email().as_ref(),
        },
    };

    Ok(HttpResponse::Ok().json(response))
}

#[derive(serde::Serialize)]
struct Response<'a> {
    token: &'a str,
    user: UserResponse<'a>,
}

#[derive(serde::Serialize)]
struct UserResponse<'a> {
    id: &'a Uuid,
    username: &'a str,
    email: &'a str,
}
