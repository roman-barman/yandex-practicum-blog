use crate::errors::{
    CreatePostError, DeletePostError, GetPostError, GetPostsListError, LoginError,
    RegisterUserError, UpdatePostError,
};
use crate::{
    AuthorizedCommand, CreatePostCommand, DeletePostCommand, GetPostCommand, GetPostsListCommand,
    LoginCommand, Pagination, Post, RegisterUserCommand, UpdatePostCommand,
};
use serde::Deserialize;

pub(crate) async fn register_user(
    address: &str,
    cmd: &RegisterUserCommand,
) -> Result<(), RegisterUserError> {
    let request = serde_json::json!({
        "username": cmd.get_username(),
        "password": cmd.get_password(),
        "email": cmd.get_email(),
    });

    let response = reqwest::Client::new()
        .post(&format!("{}/api/auth/register", address))
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::CREATED => Ok(()),
        reqwest::StatusCode::CONFLICT => Err(RegisterUserError::UsernameOrEmailExist),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY => Err(RegisterUserError::InvalidUser(
            response.json::<ErrorResponse>().await?.error,
        )),
        _ => Err(RegisterUserError::Unexpected(
            response.json::<ErrorResponse>().await?.error,
        )),
    }
}

pub(crate) async fn login(address: &str, cmd: &LoginCommand) -> Result<String, LoginError> {
    let request = serde_json::json!({
        "username": cmd.get_username(),
        "password": cmd.get_password(),
    });

    let response = reqwest::Client::new()
        .post(&format!("{}/api/auth/login", address))
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(response.json::<LoginResponse>().await?.token),
        reqwest::StatusCode::UNAUTHORIZED => Err(LoginError::InvalidUserNameOrPassword),
        _ => Err(LoginError::Unexpected(
            response.json::<ErrorResponse>().await?.error,
        )),
    }
}

pub(crate) async fn create_post(
    address: &str,
    cmd: &AuthorizedCommand<'_, CreatePostCommand>,
) -> Result<Post, CreatePostError> {
    let request = serde_json::json!({
        "title": cmd.get_command().get_title(),
        "content": cmd.get_command().get_content(),
    });

    let response = reqwest::Client::new()
        .post(&format!("{}/api/posts", address))
        .header("Authorization", format!("Bearer {}", cmd.get_token()))
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::CREATED => Ok(response.json::<Post>().await?),
        reqwest::StatusCode::UNAUTHORIZED => Err(CreatePostError::Unauthorized),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY => Err(CreatePostError::InvalidPost(
            response.json::<ErrorResponse>().await?.error,
        )),
        _ => Err(CreatePostError::Unexpected(
            response.json::<ErrorResponse>().await?.error,
        )),
    }
}

pub(crate) async fn update_post(
    address: &str,
    cmd: &AuthorizedCommand<'_, UpdatePostCommand>,
) -> Result<Post, UpdatePostError> {
    let request = serde_json::json!({
        "title": cmd.get_command().get_title(),
        "content": cmd.get_command().get_content(),
    });

    let response = reqwest::Client::new()
        .put(&format!(
            "{}/api/posts/{}",
            address,
            cmd.get_command().get_id()
        ))
        .header("Authorization", format!("Bearer {}", cmd.get_token()))
        .json(&request)
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(response.json::<Post>().await?),
        reqwest::StatusCode::UNAUTHORIZED => Err(UpdatePostError::Unauthorized),
        reqwest::StatusCode::NOT_FOUND => Err(UpdatePostError::NotFound),
        reqwest::StatusCode::FORBIDDEN => Err(UpdatePostError::Forbidden),
        reqwest::StatusCode::UNPROCESSABLE_ENTITY => Err(UpdatePostError::InvalidPost(
            response.json::<ErrorResponse>().await?.error,
        )),
        _ => Err(UpdatePostError::Unexpected(
            response.json::<ErrorResponse>().await?.error,
        )),
    }
}

pub(crate) async fn delete_post(
    address: &str,
    cmd: &AuthorizedCommand<'_, DeletePostCommand>,
) -> Result<(), DeletePostError> {
    let response = reqwest::Client::new()
        .delete(&format!(
            "{}/api/posts/{}",
            address,
            cmd.get_command().get_id()
        ))
        .header("Authorization", format!("Bearer {}", cmd.get_token()))
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::NO_CONTENT => Ok(()),
        reqwest::StatusCode::UNAUTHORIZED => Err(DeletePostError::Unauthorized),
        reqwest::StatusCode::NOT_FOUND => Err(DeletePostError::NotFound),
        reqwest::StatusCode::FORBIDDEN => Err(DeletePostError::Forbidden),
        _ => Err(DeletePostError::Unexpected(
            response.json::<ErrorResponse>().await?.error,
        )),
    }
}

pub(crate) async fn get_post(address: &str, cmd: &GetPostCommand) -> Result<Post, GetPostError> {
    let response = reqwest::Client::new()
        .get(&format!("{}/api/posts/{}", address, cmd.get_id()))
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(response.json::<Post>().await?),
        reqwest::StatusCode::NOT_FOUND => Err(GetPostError::NotFound),
        _ => Err(GetPostError::Unexpected(
            response.json::<ErrorResponse>().await?.error,
        )),
    }
}

pub(crate) async fn get_post_list(
    address: &str,
    cmd: &GetPostsListCommand,
) -> Result<Pagination<Post>, GetPostsListError> {
    let response = reqwest::Client::new()
        .get(&format!(
            "{}/api/posts?limit={}&offset={}",
            address,
            cmd.get_limit(),
            cmd.get_offset()
        ))
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let posts = response.json::<PostPagination>().await?;
            Ok(Pagination::new(
                posts.posts,
                posts.total,
                posts.limit,
                posts.offset,
            ))
        }
        _ => Err(GetPostsListError::Unexpected(
            response.json::<ErrorResponse>().await?.error,
        )),
    }
}

impl From<reqwest::Error> for GetPostsListError {
    fn from(err: reqwest::Error) -> Self {
        GetPostsListError::Unexpected(err.to_string())
    }
}

impl From<reqwest::Error> for GetPostError {
    fn from(err: reqwest::Error) -> Self {
        GetPostError::Unexpected(err.to_string())
    }
}

impl From<reqwest::Error> for DeletePostError {
    fn from(err: reqwest::Error) -> Self {
        DeletePostError::Unexpected(err.to_string())
    }
}

impl From<reqwest::Error> for UpdatePostError {
    fn from(err: reqwest::Error) -> Self {
        UpdatePostError::Unexpected(err.to_string())
    }
}

impl From<reqwest::Error> for CreatePostError {
    fn from(err: reqwest::Error) -> Self {
        CreatePostError::Unexpected(err.to_string())
    }
}

impl From<reqwest::Error> for RegisterUserError {
    fn from(err: reqwest::Error) -> Self {
        RegisterUserError::Unexpected(err.to_string())
    }
}

impl From<reqwest::Error> for LoginError {
    fn from(err: reqwest::Error) -> Self {
        LoginError::Unexpected(err.to_string())
    }
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
struct PostPagination {
    posts: Vec<Post>,
    total: usize,
    limit: usize,
    offset: usize,
}
