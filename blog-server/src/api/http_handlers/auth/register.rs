use actix_web::{HttpResponse, post};

#[post("/auth/register")]
#[tracing::instrument(name = "Register a new user")]
pub(crate) async fn register_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}
