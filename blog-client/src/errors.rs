/// Errors that can occur during user registration.
#[derive(Debug, thiserror::Error)]
pub enum RegisterUserError {
    /// Unexpected request or network error.
    #[error("request error: {0}")]
    Unexpected(String),
    /// Error when a user with the same username or email already exists.
    #[error("user with this username or email already exists")]
    UsernameOrEmailExist,
    /// Error when the user data is invalid.
    #[error("invalid user: {0}")]
    InvalidUser(String),
}

/// Errors that can occur during login.
#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    /// Unexpected request or network error.
    #[error("request error: {0}")]
    Unexpected(String),
    /// Error when the username or password is incorrect.
    #[error("invalid user name or password")]
    InvalidUserNameOrPassword,
}

/// Errors that can occur during post creation.
#[derive(Debug, thiserror::Error)]
pub enum CreatePostError {
    /// Unexpected request or network error.
    #[error("request error: {0}")]
    Unexpected(String),
    /// Error when the post data is invalid.
    #[error("invalid post: {0}")]
    InvalidPost(String),
    /// Error when the user is not authorized.
    #[error("unauthorized")]
    Unauthorized,
}

/// Errors that can occur during post update.
#[derive(Debug, thiserror::Error)]
pub enum UpdatePostError {
    /// Unexpected request or network error.
    #[error("request error: {0}")]
    Unexpected(String),
    /// Error when the post data is invalid.
    #[error("invalid post: {0}")]
    InvalidPost(String),
    /// Error when the user is not authorized.
    #[error("unauthorized")]
    Unauthorized,
    /// Error when the post is not found.
    #[error("post not found")]
    NotFound,
    /// Error when the user is forbidden from updating this post.
    #[error("forbidden")]
    Forbidden,
}

/// Errors that can occur during post deletion.
#[derive(Debug, thiserror::Error)]
pub enum DeletePostError {
    /// Unexpected request or network error.
    #[error("request error: {0}")]
    Unexpected(String),
    /// Error when the user is not authorized.
    #[error("unauthorized")]
    Unauthorized,
    /// Error when the post is not found.
    #[error("post not found")]
    NotFound,
    /// Error when the user is forbidden from deleting this post.
    #[error("forbidden")]
    Forbidden,
}

/// Errors that can occur during post retrieval.
#[derive(Debug, thiserror::Error)]
pub enum GetPostError {
    /// Unexpected request or network error.
    #[error("request error: {0}")]
    Unexpected(String),
    /// Error when the post is not found.
    #[error("post not found")]
    NotFound,
}

/// Errors that can occur during post list retrieval.
#[derive(Debug, thiserror::Error)]
pub enum GetPostsListError {
    /// Unexpected request or network error.
    #[error("request error: {0}")]
    Unexpected(String),
}
