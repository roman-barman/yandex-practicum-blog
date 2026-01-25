use crate::application::auth::{RegisterUserCommand, register_user_handler};
use actix_web::{HttpResponse, post, web};

#[post("/auth/register")]
#[tracing::instrument(name = "Register a new user")]
pub(crate) async fn register_user(request: web::Json<RegisterUserCommand>) -> HttpResponse {
    let user = register_user_handler(request.0);
    match user {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
