use crate::application::contracts::PostRepository;
use crate::domain::entities::Post;
use crate::domain::value_objects::{Content, Identification, Title, TitleError};
use html_escape::encode_text;
use std::sync::Arc;

#[tracing::instrument(name = "Handle update post command", skip(post_repo))]
pub(crate) async fn update_post_handler(
    post_id: Identification,
    user_id: Identification,
    command: UpdatePostCommand,
    post_repo: &Arc<dyn PostRepository>,
) -> Result<Post, UpdatePostError> {
    let mut post = post_repo
        .get(&post_id)
        .await
        .map_err(|e| UpdatePostError::Unexpected(e.to_string()))?
        .ok_or(UpdatePostError::NotFound)?;

    if *post.author_id() != user_id {
        return Err(UpdatePostError::NotAllowed);
    }

    let title = Title::try_from(encode_text(&command.title).to_string())?;
    let content = Content::from(encode_text(&command.content).to_string());
    post.update(title, content);
    post_repo
        .update(&post)
        .await
        .map_err(|e| UpdatePostError::Unexpected(e.to_string()))?;

    Ok(post)
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct UpdatePostCommand {
    title: String,
    content: String,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum UpdatePostError {
    #[error("post not found")]
    NotFound,
    #[error("not allowed to update post")]
    NotAllowed,
    #[error("invalid title: {0}")]
    InvalidTitle(#[from] TitleError),
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
