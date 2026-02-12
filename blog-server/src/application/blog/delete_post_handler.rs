use crate::application::contracts::PostRepository;
use crate::domain::value_objects::Identification;
use std::sync::Arc;

#[tracing::instrument(name = "Handle delete post command", skip(post_repo))]
pub(crate) async fn delete_post_handler(
    post_id: Identification,
    user_id: Identification,
    post_repo: &Arc<dyn PostRepository>,
) -> Result<(), DeletePostError> {
    let post = post_repo
        .get(&post_id)
        .await
        .map_err(|e| DeletePostError::Unexpected(e.to_string()))?
        .ok_or(DeletePostError::NotFound)?;

    if *post.author_id() != user_id {
        return Err(DeletePostError::NotAllowed);
    }

    post_repo
        .delete(&post_id)
        .await
        .map_err(|e| DeletePostError::Unexpected(e.to_string()))?;

    Ok(())
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum DeletePostError {
    #[error("post not found")]
    NotFound,
    #[error("not allowed to delete post")]
    NotAllowed,
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
