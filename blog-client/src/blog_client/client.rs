use crate::errors::{
    CreatePostError, DeletePostError, GetPostError, GetPostsListError, LoginError,
    RegisterUserError, UpdatePostError,
};
use crate::{
    AuthorizedCommand, BlogClient, CreatePostCommand, DeletePostCommand, GetPostCommand,
    GetPostsListCommand, LoginCommand, Pagination, Post, RegisterUserCommand, UpdatePostCommand,
    grpc_client, http_client,
};
use async_trait::async_trait;

pub struct Client {
    protocol: Protocol,
}

pub enum Protocol {
    Http(String),
    Grpc(String),
}

impl Client {
    pub fn new(protocol: Protocol) -> Self {
        Self { protocol }
    }
}

#[async_trait]
impl BlogClient for Client {
    async fn register_user(&self, cmd: RegisterUserCommand) -> Result<(), RegisterUserError> {
        match &self.protocol {
            Protocol::Http(address) => http_client::register_user(&address, &cmd).await,
            Protocol::Grpc(address) => grpc_client::register_user(address.clone(), &cmd).await,
        }
    }

    async fn login(&self, cmd: LoginCommand) -> Result<String, LoginError> {
        match &self.protocol {
            Protocol::Http(address) => http_client::login(&address, &cmd).await,
            Protocol::Grpc(address) => grpc_client::login(address.clone(), &cmd).await,
        }
    }

    async fn create_post(
        &self,
        cmd: AuthorizedCommand<'_, CreatePostCommand>,
    ) -> Result<Post, CreatePostError> {
        match &self.protocol {
            Protocol::Http(address) => http_client::create_post(&address, &cmd).await,
            Protocol::Grpc(address) => grpc_client::create_post(address.clone(), &cmd).await,
        }
    }

    async fn update_post(
        &self,
        cmd: AuthorizedCommand<'_, UpdatePostCommand>,
    ) -> Result<Post, UpdatePostError> {
        match &self.protocol {
            Protocol::Http(address) => http_client::update_post(&address, &cmd).await,
            Protocol::Grpc(address) => grpc_client::update_post(address.clone(), &cmd).await,
        }
    }

    async fn delete_post(
        &self,
        cmd: AuthorizedCommand<'_, DeletePostCommand>,
    ) -> Result<(), DeletePostError> {
        match &self.protocol {
            Protocol::Http(address) => http_client::delete_post(&address, &cmd).await,
            Protocol::Grpc(address) => grpc_client::delete_post(address.clone(), &cmd).await,
        }
    }

    async fn get_post(&self, cmd: GetPostCommand) -> Result<Post, GetPostError> {
        match &self.protocol {
            Protocol::Http(address) => http_client::get_post(&address, &cmd).await,
            Protocol::Grpc(address) => grpc_client::get_post(address.clone(), &cmd).await,
        }
    }

    async fn get_post_list(
        &self,
        cmd: GetPostsListCommand,
    ) -> Result<Pagination<Post>, GetPostsListError> {
        match &self.protocol {
            Protocol::Http(address) => http_client::get_post_list(&address, &cmd).await,
            Protocol::Grpc(address) => grpc_client::get_post_list(address.clone(), &cmd).await,
        }
    }
}
