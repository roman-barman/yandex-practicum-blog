mod client;

use crate::errors::{
    CreatePostError, DeletePostError, GetPostError, GetPostsListError, LoginError,
    RegisterUserError, UpdatePostError,
};
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

#[async_trait]
pub trait BlogClient {
    async fn register_user(&self, cmd: RegisterUserCommand) -> Result<(), RegisterUserError>;
    async fn login(&self, cmd: LoginCommand) -> Result<(), LoginError>;
    async fn create_post(&self, cmd: CreatePostCommand) -> Result<Post, CreatePostError>;
    async fn update_post(&self, cmd: UpdatePostCommand) -> Result<Post, UpdatePostError>;
    async fn delete_post(&self, cmd: DeletePostCommand) -> Result<(), DeletePostError>;
    async fn get_post(&self, cmd: GetPostCommand) -> Result<Post, GetPostError>;
    async fn get_post_list(
        &self,
        cmd: GetPostsListCommand,
    ) -> Result<Pagination<Post>, GetPostsListError>;
}

pub struct GetPostsListCommand {
    limit: usize,
    offset: usize,
}

pub struct GetPostCommand {
    id: Uuid,
}

pub struct DeletePostCommand {
    id: Uuid,
}

pub struct RegisterUserCommand {
    username: String,
    password: SecretString,
    email: String,
}

impl RegisterUserCommand {
    pub fn new(username: String, password: String, email: String) -> Self {
        Self {
            username,
            password: SecretString::from(password),
            email,
        }
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password.expose_secret()
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
}

pub struct LoginCommand {
    username: String,
    password: SecretString,
}

pub struct CreatePostCommand {
    title: String,
    content: String,
}

pub struct UpdatePostCommand {
    id: Uuid,
    title: String,
    content: String,
}

pub struct Post {
    id: Uuid,
    title: String,
    content: String,
    author_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub struct Pagination<T> {
    items: Vec<T>,
    total_count: usize,
    limit: usize,
    offset: usize,
}
