#[derive(Debug, thiserror::Error)]
pub enum RegisterUserError {
    #[error("request error: {0}")]
    Unexpected(String),
    #[error("user with this username or email already exists")]
    UsernameOrEmailExist,
    #[error("invalid user: {0}")]
    InvalidUser(String),
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("request error: {0}")]
    Unexpected(String),
    #[error("invalid user name or password")]
    InvalidUserNameOrPassword,
}

#[derive(Debug, thiserror::Error)]
pub enum CreatePostError {
    #[error("request error: {0}")]
    Unexpected(String),
    #[error("invalid post: {0}")]
    InvalidPost(String),
    #[error("unauthorized")]
    Unauthorized,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdatePostError {
    #[error("request error: {0}")]
    Unexpected(String),
    #[error("invalid post: {0}")]
    InvalidPost(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("post not found")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
}

#[derive(Debug, thiserror::Error)]
pub enum DeletePostError {
    #[error("request error: {0}")]
    Unexpected(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("post not found")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
}

#[derive(Debug, thiserror::Error)]
pub enum GetPostError {
    #[error("request error: {0}")]
    Unexpected(String),
    #[error("post not found")]
    NotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum GetPostsListError {
    #[error("request error: {0}")]
    Unexpected(String),
}
