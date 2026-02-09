mod client;

pub use client::{Client, Errors, Protocol};
use std::fmt::{Display, Formatter};

use crate::errors::{
    CreatePostError, DeletePostError, GetPostError, GetPostsListError, LoginError,
    RegisterUserError, UpdatePostError,
};
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use uuid::Uuid;

/// Trait representing a client for the blog system.
///
/// It provides methods for user management and post operations.
#[async_trait]
pub trait BlogClient {
    /// Registers a new user.
    async fn register_user(&mut self, cmd: RegisterUserCommand) -> Result<(), RegisterUserError>;
    /// Authenticates a user and returns a JWT token.
    async fn login(&mut self, cmd: LoginCommand) -> Result<String, LoginError>;
    /// Creates a new post. Requires authorization.
    async fn create_post(
        &mut self,
        cmd: AuthorizedCommand<'_, CreatePostCommand>,
    ) -> Result<Post, CreatePostError>;
    /// Updates an existing post. Requires authorization.
    async fn update_post(
        &mut self,
        cmd: AuthorizedCommand<'_, UpdatePostCommand>,
    ) -> Result<Post, UpdatePostError>;
    /// Deletes a post. Requires authorization.
    async fn delete_post(
        &mut self,
        cmd: AuthorizedCommand<'_, DeletePostCommand>,
    ) -> Result<(), DeletePostError>;
    /// Retrieves a single post by its ID.
    async fn get_post(&mut self, cmd: GetPostCommand) -> Result<Post, GetPostError>;
    /// Retrieves a paginated list of posts.
    async fn get_post_list(
        &mut self,
        cmd: GetPostsListCommand,
    ) -> Result<Pagination<Post>, GetPostsListError>;
}

/// Command for retrieving a list of posts with pagination.
pub struct GetPostsListCommand {
    limit: usize,
    offset: usize,
}

impl GetPostsListCommand {
    /// Creates a new `GetPostsListCommand`.
    pub fn new(limit: usize, offset: usize) -> Self {
        Self { limit, offset }
    }

    /// Returns the limit.
    pub fn get_limit(&self) -> usize {
        self.limit
    }
    /// Returns the offset.
    pub fn get_offset(&self) -> usize {
        self.offset
    }
}

/// Command for retrieving a specific post by its ID.
pub struct GetPostCommand {
    id: Uuid,
}

impl GetPostCommand {
    /// Creates a new `GetPostCommand`.
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    /// Returns the post ID.
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}

/// Command for deleting a post by its ID.
pub struct DeletePostCommand {
    id: Uuid,
}

impl DeletePostCommand {
    /// Creates a new `DeletePostCommand`.
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }

    /// Returns the post ID.
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}

/// Command for registering a new user.
pub struct RegisterUserCommand {
    username: String,
    password: SecretString,
    email: String,
}

impl RegisterUserCommand {
    /// Creates a new `RegisterUserCommand`.
    pub fn new(username: String, password: String, email: String) -> Self {
        Self {
            username,
            password: SecretString::from(password),
            email,
        }
    }

    /// Returns the username.
    pub fn get_username(&self) -> &str {
        &self.username
    }
    /// Returns the password (exposed from secret string).
    pub fn get_password(&self) -> &str {
        self.password.expose_secret()
    }
    /// Returns the email.
    pub fn get_email(&self) -> &str {
        &self.email
    }
}

/// Command for logging in a user.
pub struct LoginCommand {
    username: String,
    password: SecretString,
}

impl LoginCommand {
    /// Creates a new `LoginCommand`.
    pub fn new(username: String, password: String) -> Self {
        Self {
            username,
            password: SecretString::from(password),
        }
    }
    /// Returns the username.
    pub fn get_username(&self) -> &str {
        &self.username
    }
    /// Returns the password (exposed from secret string).
    pub fn get_password(&self) -> &str {
        self.password.expose_secret()
    }
}

/// Command for creating a new post.
pub struct CreatePostCommand {
    title: String,
    content: String,
}

impl CreatePostCommand {
    /// Creates a new `CreatePostCommand`.
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
    }

    /// Returns the title.
    pub fn get_title(&self) -> &str {
        &self.title
    }
    /// Returns the content.
    pub fn get_content(&self) -> &str {
        &self.content
    }
}

/// Command for updating an existing post.
pub struct UpdatePostCommand {
    id: Uuid,
    title: String,
    content: String,
}

impl UpdatePostCommand {
    /// Creates a new `UpdatePostCommand`.
    pub fn new(id: Uuid, title: String, content: String) -> Self {
        Self { id, title, content }
    }

    /// Returns the post ID.
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
    /// Returns the updated title.
    pub fn get_title(&self) -> &str {
        &self.title
    }
    /// Returns the updated content.
    pub fn get_content(&self) -> &str {
        &self.content
    }
}

/// Represents a blog post.
#[derive(Deserialize, Debug)]
pub struct Post {
    id: Uuid,
    title: String,
    content: String,
    user_id: Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Post {
    pub(crate) fn new(
        id: Uuid,
        title: String,
        content: String,
        user_id: Uuid,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            title,
            content,
            user_id,
            created_at,
            updated_at,
        }
    }
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Content: {}", self.content)?;
        writeln!(f, "Author ID: {}", self.user_id)?;
        writeln!(f, "Created at: {}", self.created_at)?;
        writeln!(f, "Updated at: {}", self.updated_at)?;
        Ok(())
    }
}

/// Wrapper for commands that require an authorization token.
pub struct AuthorizedCommand<'a, T> {
    command: T,
    token: &'a str,
}

impl<'a, T> AuthorizedCommand<'a, T> {
    /// Creates a new `AuthorizedCommand`.
    pub fn new(command: T, token: &'a str) -> Self {
        Self { command, token }
    }

    /// Returns the wrapped command.
    pub fn get_command(&self) -> &T {
        &self.command
    }

    /// Returns the authorization token.
    pub fn get_token(&self) -> &str {
        self.token
    }
}

/// Represents a paginated list of items.
pub struct Pagination<T> {
    items: Vec<T>,
    total_count: usize,
    limit: usize,
    offset: usize,
}

impl<T> Pagination<T> {
    /// Creates a new `Pagination` result.
    pub fn new(items: Vec<T>, total_count: usize, limit: usize, offset: usize) -> Self {
        Self {
            items,
            total_count,
            limit,
            offset,
        }
    }
}

impl<T: Display> Display for Pagination<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Total count: {}", self.total_count)?;
        writeln!(f, "Limit: {}", self.limit)?;
        writeln!(f, "Offset: {}", self.offset)?;
        for item in &self.items {
            writeln!(f, "{}", item)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_post() -> () {
        let response = r#"{
  "id": "204ff4d7-ea2e-4e7c-becb-4eb46747d81d",
  "title": "Hello world",
  "content": "content",
  "user_id": "a25ff634-c48e-48c9-8f15-f19cd1c26884",
  "created_at": "2026-02-08T15:39:47.064297652Z",
  "updated_at": "2026-02-08T15:39:47.064300304Z"
}"#;
        let post = serde_json::from_str::<Post>(response);
        println!("Deserialized post: {:?}", post);
        assert!(post.is_ok());
    }
}
