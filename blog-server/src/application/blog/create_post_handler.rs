use crate::domain::entities::Blog;
use crate::domain::value_objects::{Content, Identification, Title, TitleError};
use html_escape::encode_text;

#[tracing::instrument(name = "Handle create post command")]
pub(crate) fn create_post_handler(
    user_id: Identification,
    command: CreatePostCommand,
) -> Result<Blog, CreatePostError> {
    let title = Title::try_from(encode_text(&command.title).to_string())?;
    let content = Content::from(encode_text(&command.content).to_string());

    Ok(Blog::new(title, content, user_id))
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
}
