use crate::application::contracts::PostRepository;
use crate::domain::entities::Post;
use std::sync::Arc;

#[tracing::instrument(name = "Handle get post list command", skip(post_repo))]
pub(crate) async fn get_post_list_handler(
    limit: usize,
    offset: usize,
    post_repo: &Arc<dyn PostRepository>,
) -> Result<(Vec<Post>, usize), GetPostListError> {
    post_repo
        .list(limit, offset)
        .await
        .map_err(|e| GetPostListError::Unexpected(e.to_string()))
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum GetPostListError {
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
