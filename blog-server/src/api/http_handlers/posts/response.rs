use crate::domain::entities::Post;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub(crate) struct PostResponse<'a> {
    id: &'a Uuid,
    title: &'a str,
    content: &'a str,
    user_id: &'a Uuid,
    created_at: &'a chrono::DateTime<chrono::Utc>,
    updated_at: &'a chrono::DateTime<chrono::Utc>,
}

impl<'a> PostResponse<'a> {
    pub(crate) fn from(post: &'a Post) -> PostResponse<'a> {
        Self {
            id: post.id().as_ref(),
            title: post.title().as_ref(),
            content: post.content().as_ref(),
            user_id: post.author_id().as_ref(),
            created_at: post.created_at().as_ref(),
            updated_at: post.updated_at().as_ref(),
        }
    }
}
