use crate::errors::{
    CreatePostError, DeletePostError, GetPostError, GetPostsListError, LoginError,
    RegisterUserError, UpdatePostError,
};
use crate::grpc_client::blog_service_client::BlogServiceClient;
use std::str::FromStr;
use tonic::metadata::MetadataValue;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::{Code, Request, Status};
use uuid::Uuid;

tonic::include_proto!("blog");

pub(crate) struct GrpcClient {
    client: BlogServiceClient<tonic::transport::Channel>,
}

impl GrpcClient {
    pub(crate) async fn new(address: String) -> Result<Self, tonic::transport::Error> {
        Ok(Self {
            client: BlogServiceClient::connect(address).await?,
        })
    }

    pub(crate) async fn register_user(
        &mut self,
        cmd: &crate::RegisterUserCommand,
    ) -> Result<(), RegisterUserError> {
        let request: Request<RegisterUserCommand> = cmd.into();
        let _ = self.client.register_user(request).await?;
        Ok(())
    }

    pub(crate) async fn login(&mut self, cmd: &crate::LoginCommand) -> Result<String, LoginError> {
        let request: Request<VerifyUserCommand> = cmd.into();
        Ok(self.client.login(request).await?.into_inner().token)
    }

    pub(crate) async fn create_post(
        &mut self,
        cmd: &crate::AuthorizedCommand<'_, crate::CreatePostCommand>,
    ) -> Result<crate::Post, CreatePostError> {
        let request: Request<CreatePostCommand> = cmd
            .try_into()
            .map_err(|e: InvalidMetadataValue| CreatePostError::Unexpected(e.to_string()))?;
        self.client
            .create_post(request)
            .await?
            .into_inner()
            .try_into()
            .map_err(CreatePostError::Unexpected)
    }

    pub(crate) async fn update_post(
        &mut self,
        cmd: &crate::AuthorizedCommand<'_, crate::UpdatePostCommand>,
    ) -> Result<crate::Post, UpdatePostError> {
        let request: Request<UpdatePostCommand> = cmd
            .try_into()
            .map_err(|e: InvalidMetadataValue| UpdatePostError::Unexpected(e.to_string()))?;
        self.client
            .update_post(request)
            .await?
            .into_inner()
            .try_into()
            .map_err(UpdatePostError::Unexpected)
    }

    pub(crate) async fn delete_post(
        &mut self,
        cmd: &crate::AuthorizedCommand<'_, crate::DeletePostCommand>,
    ) -> Result<(), DeletePostError> {
        let request: Request<DeletePostCommand> = cmd
            .try_into()
            .map_err(|e: InvalidMetadataValue| DeletePostError::Unexpected(e.to_string()))?;
        let _ = self.client.delete_post(request).await?;
        Ok(())
    }

    pub(crate) async fn get_post(
        &mut self,
        cmd: &crate::GetPostCommand,
    ) -> Result<crate::Post, GetPostError> {
        let request: Request<GetPostCommand> = cmd.into();
        self.client
            .get_post(request)
            .await?
            .into_inner()
            .try_into()
            .map_err(GetPostError::Unexpected)
    }

    pub(crate) async fn get_post_list(
        &mut self,
        cmd: &crate::GetPostsListCommand,
    ) -> Result<crate::Pagination<crate::Post>, GetPostsListError> {
        let request: Request<GetPostListCommand> = cmd.into();
        self.client
            .get_post_list(request)
            .await?
            .into_inner()
            .try_into()
            .map_err(GetPostsListError::Unexpected)
    }
}

impl TryFrom<GetPostListResult> for crate::Pagination<crate::Post> {
    type Error = String;
    fn try_from(value: GetPostListResult) -> Result<Self, Self::Error> {
        let mut posts = Vec::with_capacity(value.post.len());
        for post in value.post {
            posts.push(crate::Post::try_from(post)?);
        }
        Ok(crate::Pagination::new(
            posts,
            value.total as usize,
            value.offset as usize,
            value.limit as usize,
        ))
    }
}

impl From<Status> for GetPostsListError {
    fn from(status: Status) -> Self {
        GetPostsListError::Unexpected(status.message().to_string())
    }
}

impl From<&crate::GetPostsListCommand> for Request<GetPostListCommand> {
    fn from(cmd: &crate::GetPostsListCommand) -> Self {
        Request::new(GetPostListCommand {
            limit: cmd.get_limit() as u32,
            offset: cmd.get_offset() as u32,
        })
    }
}

impl From<Status> for GetPostError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::NotFound => GetPostError::NotFound,
            _ => GetPostError::Unexpected(status.message().to_string()),
        }
    }
}

impl From<&crate::GetPostCommand> for Request<GetPostCommand> {
    fn from(cmd: &crate::GetPostCommand) -> Self {
        Request::new(GetPostCommand {
            id: cmd.get_id().to_string(),
        })
    }
}

impl From<Status> for DeletePostError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::Unauthenticated => DeletePostError::Unauthorized,
            Code::NotFound => DeletePostError::NotFound,
            Code::PermissionDenied => DeletePostError::Forbidden,
            _ => DeletePostError::Unexpected(status.message().to_string()),
        }
    }
}

impl TryFrom<&crate::AuthorizedCommand<'_, crate::DeletePostCommand>>
    for Request<DeletePostCommand>
{
    type Error = InvalidMetadataValue;
    fn try_from(
        cmd: &crate::AuthorizedCommand<'_, crate::DeletePostCommand>,
    ) -> Result<Self, Self::Error> {
        let mut request = Request::new(DeletePostCommand {
            id: cmd.get_command().get_id().to_string(),
        });
        add_token(&mut request, cmd.get_token())?;
        Ok(request)
    }
}

impl From<Status> for UpdatePostError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::InvalidArgument => UpdatePostError::InvalidPost(status.message().to_string()),
            Code::Unauthenticated => UpdatePostError::Unauthorized,
            Code::NotFound => UpdatePostError::NotFound,
            Code::PermissionDenied => UpdatePostError::Forbidden,
            _ => UpdatePostError::Unexpected(status.message().to_string()),
        }
    }
}

impl TryFrom<&crate::AuthorizedCommand<'_, crate::UpdatePostCommand>>
    for Request<UpdatePostCommand>
{
    type Error = InvalidMetadataValue;
    fn try_from(
        cmd: &crate::AuthorizedCommand<'_, crate::UpdatePostCommand>,
    ) -> Result<Self, Self::Error> {
        let mut request = Request::new(UpdatePostCommand {
            id: cmd.get_command().get_id().to_string(),
            title: cmd.get_command().get_title().to_string(),
            content: cmd.get_command().get_content().to_string(),
        });
        add_token(&mut request, cmd.get_token())?;
        Ok(request)
    }
}

impl TryFrom<Post> for crate::Post {
    type Error = String;
    fn try_from(value: Post) -> Result<Self, Self::Error> {
        let id = Uuid::try_parse(value.id.as_str()).map_err(|_| "Invalid UUID")?;
        let title = value.title;
        let content = value.content;
        let author_id = Uuid::try_parse(value.author_id.as_str()).map_err(|_| "Invalid UUID")?;
        let created_at = value.created_at.ok_or("Missing created_at")?;
        let updated_at = value.updated_at.ok_or("Missing updated_at")?;
        let created_at =
            chrono::DateTime::from_timestamp(created_at.seconds, created_at.nanos as u32)
                .unwrap_or_default();
        let updated_at =
            chrono::DateTime::from_timestamp(updated_at.seconds, updated_at.nanos as u32)
                .unwrap_or_default();

        Ok(crate::Post::new(
            id, title, content, author_id, created_at, updated_at,
        ))
    }
}

impl From<Status> for CreatePostError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::InvalidArgument => CreatePostError::InvalidPost(status.message().to_string()),
            Code::Unauthenticated => CreatePostError::Unauthorized,
            _ => CreatePostError::Unexpected(status.message().to_string()),
        }
    }
}

impl TryFrom<&crate::AuthorizedCommand<'_, crate::CreatePostCommand>>
    for Request<CreatePostCommand>
{
    type Error = InvalidMetadataValue;
    fn try_from(
        cmd: &crate::AuthorizedCommand<'_, crate::CreatePostCommand>,
    ) -> Result<Self, Self::Error> {
        let mut request = Request::new(CreatePostCommand {
            title: cmd.get_command().get_title().to_string(),
            content: cmd.get_command().get_content().to_string(),
        });
        add_token(&mut request, cmd.get_token())?;

        Ok(request)
    }
}

impl From<Status> for LoginError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::NotFound | Code::InvalidArgument => LoginError::InvalidUserNameOrPassword,
            _ => LoginError::Unexpected(status.message().to_string()),
        }
    }
}

impl From<&crate::LoginCommand> for Request<VerifyUserCommand> {
    fn from(cmd: &crate::LoginCommand) -> Self {
        Request::new(VerifyUserCommand {
            username: cmd.get_username().to_string(),
            password: cmd.get_password().to_string(),
        })
    }
}

impl From<Status> for RegisterUserError {
    fn from(status: Status) -> Self {
        match status.code() {
            Code::AlreadyExists => RegisterUserError::UsernameOrEmailExist,
            Code::InvalidArgument => RegisterUserError::InvalidUser(status.message().to_string()),
            _ => RegisterUserError::Unexpected(status.message().to_string()),
        }
    }
}

impl From<&crate::RegisterUserCommand> for Request<RegisterUserCommand> {
    fn from(cmd: &crate::RegisterUserCommand) -> Self {
        Request::new(RegisterUserCommand {
            username: cmd.get_username().to_string(),
            password: cmd.get_password().to_string(),
            email: cmd.get_email().to_string(),
        })
    }
}

fn add_token<T>(request: &mut Request<T>, token: &str) -> Result<(), InvalidMetadataValue> {
    request
        .metadata_mut()
        .insert("authorization", MetadataValue::from_str(token)?);
    Ok(())
}
