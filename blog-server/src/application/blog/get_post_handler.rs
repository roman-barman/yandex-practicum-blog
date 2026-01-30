use crate::application::contracts::PostRepository;
use crate::domain::entities::Post;
use crate::domain::value_objects::Identification;
use std::sync::Arc;

#[tracing::instrument(name = "Handle get post command", skip(post_repo))]
pub(crate) async fn get_post_handler(
    post_id: Identification,
    post_repo: &Arc<dyn PostRepository>,
) -> Result<Post, GetPostError> {
    let post = post_repo
        .get(&post_id)
        .await
        .map_err(|e| GetPostError::Unexpected(e.to_string()))?
        .ok_or(GetPostError::NotFound)?;
    Ok(post)
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum GetPostError {
    #[error("post not found")]
    NotFound,
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
