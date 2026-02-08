mod client;

use crate::errors::{
    CreatePostError, DeletePostError, GetPostError, GetPostsListError, LoginError,
    RegisterUserError, UpdatePostError,
};
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use uuid::Uuid;

#[async_trait]
pub trait BlogClient {
    async fn register_user(&mut self, cmd: RegisterUserCommand) -> Result<(), RegisterUserError>;
    async fn login(&mut self, cmd: LoginCommand) -> Result<String, LoginError>;
    async fn create_post(
        &mut self,
        cmd: AuthorizedCommand<'_, CreatePostCommand>,
    ) -> Result<Post, CreatePostError>;
    async fn update_post(
        &mut self,
        cmd: AuthorizedCommand<'_, UpdatePostCommand>,
    ) -> Result<Post, UpdatePostError>;
    async fn delete_post(
        &mut self,
        cmd: AuthorizedCommand<'_, DeletePostCommand>,
    ) -> Result<(), DeletePostError>;
    async fn get_post(&mut self, cmd: GetPostCommand) -> Result<Post, GetPostError>;
    async fn get_post_list(
        &mut self,
        cmd: GetPostsListCommand,
    ) -> Result<Pagination<Post>, GetPostsListError>;
}

pub struct GetPostsListCommand {
    limit: usize,
    offset: usize,
}

impl GetPostsListCommand {
    pub fn new(limit: usize, offset: usize) -> Self {
        Self { limit, offset }
    }

    pub fn get_limit(&self) -> usize {
        self.limit
    }
    pub fn get_offset(&self) -> usize {
        self.offset
    }
}

pub struct GetPostCommand {
    id: Uuid,
}

impl GetPostCommand {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}

pub struct DeletePostCommand {
    id: Uuid,
}

impl DeletePostCommand {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
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

impl LoginCommand {
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password: SecretString::from(password),
        }
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        self.password.expose_secret()
    }
}

pub struct CreatePostCommand {
    title: String,
    content: String,
}

impl CreatePostCommand {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
}

pub struct UpdatePostCommand {
    id: Uuid,
    title: String,
    content: String,
}

impl UpdatePostCommand {
    pub fn new(id: Uuid, title: String, content: String) -> Self {
        Self { id, title, content }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
}

#[derive(Deserialize)]
pub struct Post {
    id: Uuid,
    title: String,
    content: String,
    author_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Post {
    pub(crate) fn new(
        id: Uuid,
        title: String,
        content: String,
        author_id: Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            author_id,
            created_at,
            updated_at,
        }
    }
}

pub struct AuthorizedCommand<'a, T> {
    command: T,
    token: &'a str,
}

impl<'a, T> AuthorizedCommand<'a, T> {
    pub fn new(command: T, token: &'a str) -> Self {
        Self { command, token }
    }

    pub fn get_command(&self) -> &T {
        &self.command
    }

    pub fn get_token(&self) -> &str {
        self.token
    }
}

pub struct Pagination<T> {
    items: Vec<T>,
    total_count: usize,
    limit: usize,
    offset: usize,
}

impl<T> Pagination<T> {
    pub fn new(items: Vec<T>, total_count: usize, limit: usize, offset: usize) -> Self {
        Self {
            items,
            total_count,
            limit,
            offset,
        }
    }
}
