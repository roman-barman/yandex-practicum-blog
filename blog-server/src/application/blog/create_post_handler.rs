use crate::application::contracts::PostRepository;
use crate::domain::entities::Post;
use crate::domain::value_objects::{Content, Identification, Title, TitleError};
use html_escape::encode_text;
use std::sync::Arc;

#[tracing::instrument(name = "Handle create post command", skip(post_repo))]
pub(crate) async fn create_post_handler(
    user_id: Identification,
    command: CreatePostCommand,
    post_repo: &Arc<dyn PostRepository>,
) -> Result<Post, CreatePostError> {
    let title = Title::try_from(encode_text(&command.title).to_string())?;
    let content = Content::from(encode_text(&command.content).to_string());
    let post = Post::new(title, content, user_id);
    post_repo
        .create(&post)
        .await
        .map_err(|err| CreatePostError::Unexpected(err.to_string()))?;
    Ok(post)
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct CreatePostCommand {
    title: String,
    content: String,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub(crate) enum CreatePostError {
    #[error("invalid title: {0}")]
    InvalidTitle(#[from] TitleError),
    #[error("unexpected error: {0}")]
    Unexpected(String),
}
