#[derive(Debug, thiserror::Error)]
pub enum RegisterUserError {
    #[error("request error: {0}")]
    Unexpected(String),
    #[error("user with this username or email already exists")]
    UsernameOrEmailExist,
    #[error("invalid user: {0}")]
    InvalidUser(String),
}
pub enum LoginError {}
pub enum CreatePostError {}
pub enum UpdatePostError {}
pub enum DeletePostError {}
pub enum GetPostError {}
pub enum GetPostsListError {}
