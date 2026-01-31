use crate::api::errors::ApiError;
use crate::api::http_handlers::posts::PostResponse;
use crate::application::blog::get_post_handler;
use crate::application::contracts::PostRepository;
use crate::domain::value_objects::Identification;
use actix_web::{HttpResponse, get, web};
use std::sync::Arc;
use uuid::Uuid;

#[get("/posts/{id}")]
#[tracing::instrument(name = "Get post", skip(post_repo))]
pub(crate) async fn get_post(
    path: web::Path<Uuid>,
    post_repo: web::Data<Arc<dyn PostRepository>>,
) -> Result<HttpResponse, ApiError> {
    let post =
        get_post_handler(Identification::from(path.into_inner()), post_repo.get_ref()).await?;
    Ok(HttpResponse::Ok().json(PostResponse::from(&post)))
}
