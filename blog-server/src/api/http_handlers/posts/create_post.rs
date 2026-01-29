use crate::api::errors::ApiError;
use crate::api::extractors::AuthenticatedUser;
use crate::application::blog::{CreatePostCommand, create_post_handler};
use actix_web::{HttpResponse, post, web};
use uuid::Uuid;

#[post("")]
#[tracing::instrument(name = "Create post")]
pub(crate) async fn create_post(
    user: AuthenticatedUser,
    request: web::Json<CreatePostCommand>,
) -> Result<HttpResponse, ApiError> {
    let blog = create_post_handler(user.id().clone(), request.into_inner())?;
    let response = Response {
        id: blog.id().as_ref(),
        title: blog.title().as_ref(),
        content: blog.content().as_ref(),
        user_id: blog.author_id().as_ref(),
        created_at: blog.created_at().as_ref(),
        updated_at: blog.updated_at().as_ref(),
    };
    Ok(HttpResponse::Created()
        .append_header(("Location", format!("/api/posts/{}", blog.id().as_ref())))
        .json(response))
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
