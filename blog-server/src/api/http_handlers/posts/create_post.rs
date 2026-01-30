use crate::api::errors::ApiError;
use crate::api::extractors::AuthenticatedUser;
use crate::api::http_handlers::posts::Response;
use crate::application::blog::{CreatePostCommand, create_post_handler};
use crate::application::contracts::PostRepository;
use actix_web::{HttpResponse, post, web};
use std::sync::Arc;

#[post("")]
#[tracing::instrument(name = "Create post", skip(post_repo))]
pub(crate) async fn create_post(
    user: AuthenticatedUser,
    request: web::Json<CreatePostCommand>,
    post_repo: web::Data<Arc<dyn PostRepository>>,
) -> Result<HttpResponse, ApiError> {
    let post = create_post_handler(user.into(), request.into_inner(), &post_repo).await?;
    Ok(HttpResponse::Created()
        .append_header(("Location", format!("/api/posts/{}", post.id().as_ref())))
        .json(Response::from(&post)))
}
