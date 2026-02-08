use crate::errors::{
    CreatePostError, DeletePostError, GetPostError, GetPostsListError, LoginError,
    RegisterUserError, UpdatePostError,
};
use crate::grpc_client::GrpcClient;
use crate::http_client::HttpClient;
use crate::{
    AuthorizedCommand, BlogClient, CreatePostCommand, DeletePostCommand, GetPostCommand,
    GetPostsListCommand, LoginCommand, Pagination, Post, RegisterUserCommand, UpdatePostCommand,
};
use async_trait::async_trait;

pub struct Client {
    client_mode: ClientMode,
}

pub enum Protocol {
    Http(String),
    Grpc(String),
}

enum ClientMode {
    Http(HttpClient),
    Grpc(GrpcClient),
}

impl Client {
    pub async fn new(protocol: Protocol) -> Result<Self, Errors> {
        match protocol {
            Protocol::Http(address) => Ok(Self {
                client_mode: ClientMode::Http(HttpClient::new(address)),
            }),
            Protocol::Grpc(address) => Ok(Self {
                client_mode: ClientMode::Grpc(GrpcClient::new(address).await?),
            }),
        }
    }
}

#[async_trait]
impl BlogClient for Client {
    async fn register_user(&mut self, cmd: RegisterUserCommand) -> Result<(), RegisterUserError> {
        match self.client_mode {
            ClientMode::Http(ref client) => client.register_user(&cmd).await,
            ClientMode::Grpc(ref mut client) => client.register_user(&cmd).await,
        }
    }

    async fn login(&mut self, cmd: LoginCommand) -> Result<String, LoginError> {
        match self.client_mode {
            ClientMode::Http(ref client) => client.login(&cmd).await,
            ClientMode::Grpc(ref mut client) => client.login(&cmd).await,
        }
    }

    async fn create_post(
        &mut self,
        cmd: AuthorizedCommand<'_, CreatePostCommand>,
    ) -> Result<Post, CreatePostError> {
        match self.client_mode {
            ClientMode::Http(ref client) => client.create_post(&cmd).await,
            ClientMode::Grpc(ref mut client) => client.create_post(&cmd).await,
        }
    }

    async fn update_post(
        &mut self,
        cmd: AuthorizedCommand<'_, UpdatePostCommand>,
    ) -> Result<Post, UpdatePostError> {
        match self.client_mode {
            ClientMode::Http(ref client) => client.update_post(&cmd).await,
            ClientMode::Grpc(ref mut client) => client.update_post(&cmd).await,
        }
    }

    async fn delete_post(
        &mut self,
        cmd: AuthorizedCommand<'_, DeletePostCommand>,
    ) -> Result<(), DeletePostError> {
        match self.client_mode {
            ClientMode::Http(ref client) => client.delete_post(&cmd).await,
            ClientMode::Grpc(ref mut client) => client.delete_post(&cmd).await,
        }
    }

    async fn get_post(&mut self, cmd: GetPostCommand) -> Result<Post, GetPostError> {
        match self.client_mode {
            ClientMode::Http(ref client) => client.get_post(&cmd).await,
            ClientMode::Grpc(ref mut client) => client.get_post(&cmd).await,
        }
    }

    async fn get_post_list(
        &mut self,
        cmd: GetPostsListCommand,
    ) -> Result<Pagination<Post>, GetPostsListError> {
        match self.client_mode {
            ClientMode::Http(ref client) => client.get_post_list(&cmd).await,
            ClientMode::Grpc(ref mut client) => client.get_post_list(&cmd).await,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("connection error: {0}")]
    ConnectionError(#[from] tonic::transport::Error),
}
