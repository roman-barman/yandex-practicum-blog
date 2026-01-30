use crate::api::errors::ApiError;
use crate::api::extractors::AuthenticatedUser;
use crate::application::blog::delete_post_handler;
use crate::application::contracts::PostRepository;
use crate::domain::value_objects::Identification;
use actix_web::{HttpResponse, delete, web};
use std::sync::Arc;
use uuid::Uuid;

#[delete("/{id}")]
#[tracing::instrument(name = "Delete post", skip(post_repo))]
pub(crate) async fn delete_post(
    path: web::Path<Uuid>,
    user: AuthenticatedUser,
    post_repo: web::Data<Arc<dyn PostRepository>>,
) -> Result<HttpResponse, ApiError> {
    delete_post_handler(
        Identification::from(path.into_inner()),
        user.into(),
        &post_repo,
    )
    .await?;
    Ok(HttpResponse::NoContent().finish())
}
