use crate::api::errors::ApiError;
use crate::api::extractors::AuthenticatedUser;
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
    let response = Response {
        id: post.id().as_ref(),
        title: post.title().as_ref(),
        content: post.content().as_ref(),
        user_id: post.author_id().as_ref(),
        created_at: post.created_at().as_ref(),
        updated_at: post.updated_at().as_ref(),
    };
    Ok(HttpResponse::Ok().json(response))
}

#[derive(serde::Serialize)]
struct Response<'a> {
    id: &'a Uuid,
    title: &'a str,
    content: &'a str,
    user_id: &'a Uuid,
    created_at: &'a chrono::DateTime<chrono::Utc>,
    updated_at: &'a chrono::DateTime<chrono::Utc>,
}
