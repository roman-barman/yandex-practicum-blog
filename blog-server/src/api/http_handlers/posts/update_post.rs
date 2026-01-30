use crate::api::errors::ApiError;
use crate::api::extractors::AuthenticatedUser;
use crate::api::http_handlers::posts::response::Response;
use crate::application::blog::{UpdatePostCommand, update_post_handler};
use crate::application::contracts::PostRepository;
use crate::domain::value_objects::Identification;
use actix_web::{HttpResponse, put, web};
use std::sync::Arc;
use uuid::Uuid;

#[put("/{id}")]
#[tracing::instrument(name = "Update post", skip(post_repo))]
pub(crate) async fn update_post(
    path: web::Path<Uuid>,
    user: AuthenticatedUser,
    request: web::Json<UpdatePostCommand>,
    post_repo: web::Data<Arc<dyn PostRepository>>,
) -> Result<HttpResponse, ApiError> {
    let post = update_post_handler(
        Identification::from(path.into_inner()),
        user.into(),
        request.into_inner(),
        &post_repo,
    )
    .await?;
    Ok(HttpResponse::Ok().json(Response::from(&post)))
}
