use crate::api::grpc_handlers::blog::blog_service_server::BlogService;
use crate::api::grpc_handlers::blog::{
    CreatePostCommand, DeletePostCommand, GetPostCommand, GetPostListCommand, GetPostListResult,
    Post, RegisterUserCommand, UpdatePostCommand, User, VerifyUserCommand, VerifyUserResult,
};
use crate::application::auth::{
    RegisterUserError, VerifyUserError, register_user_handler, verify_user_handler,
};
use crate::application::blog::{
    CreatePostError, DeletePostError, GetPostError, GetPostListError, UpdatePostError,
    create_post_handler, delete_post_handler, get_post_handler, get_post_list_handler,
    update_post_handler,
};
use crate::application::contracts::{PostRepository, UserRepository};
use crate::domain::value_objects::{DateTime, Identification};
use crate::infrastructure::JwtService;
use secrecy::SecretString;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub(crate) struct GrpcBlogService {
    user_repo: Arc<dyn UserRepository>,
    post_repo: Arc<dyn PostRepository>,
    jwt_service: Arc<JwtService>,
}

impl GrpcBlogService {
    pub(crate) fn new(
        user_repo: Arc<dyn UserRepository>,
        post_repo: Arc<dyn PostRepository>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            user_repo,
            post_repo,
            jwt_service,
        }
    }
}

#[tonic::async_trait]
impl BlogService for GrpcBlogService {
    #[tracing::instrument(name = "GRPC register a new user", skip(self))]
    async fn register_user(
        &self,
        request: Request<RegisterUserCommand>,
    ) -> Result<Response<User>, Status> {
        let user = register_user_handler(request.into(), &self.user_repo).await?;
        Ok(Response::new(User {
            id: user.id().as_ref().to_string(),
            username: user.username().as_ref().to_string(),
            email: user.email().as_ref().to_string(),
        }))
    }

    #[tracing::instrument(name = "GRPC login", skip(self))]
    async fn login(
        &self,
        request: Request<VerifyUserCommand>,
    ) -> Result<Response<VerifyUserResult>, Status> {
        let user = verify_user_handler(request.into(), &self.user_repo).await?;
        let token = self
            .jwt_service
            .generate_jwt(&user)
            .map_err(|_| Status::internal("internal error"))?;
        Ok(Response::new(VerifyUserResult {
            token,
            user: Some(User {
                id: user.id().as_ref().to_string(),
                username: user.username().as_ref().to_string(),
                email: user.email().as_ref().to_string(),
            }),
        }))
    }

    #[tracing::instrument(name = "GRPC create post", skip(self))]
    async fn create_post(
        &self,
        request: Request<CreatePostCommand>,
    ) -> Result<Response<Post>, Status> {
        let author_id = validate_credentials(&request, &self.jwt_service)?;
        let post = create_post_handler(author_id, request.into(), &self.post_repo).await?;
        Ok(Response::new(Post {
            id: post.id().as_ref().to_string(),
            title: post.title().as_ref().to_string(),
            content: post.content().as_ref().to_string(),
            author_id: post.author_id().as_ref().to_string(),
            created_at: Some(post.created_at().into()),
            updated_at: Some(post.updated_at().into()),
        }))
    }

    #[tracing::instrument(name = "GRPC update post", skip(self))]
    async fn update_post(
        &self,
        request: Request<UpdatePostCommand>,
    ) -> Result<Response<Post>, Status> {
        let author_id = validate_credentials(&request, &self.jwt_service)?;
        let post_id: Identification = Uuid::try_parse(request.get_ref().id.as_str())
            .map_err(|_| Status::invalid_argument("invalid post id"))?
            .into();
        let post = update_post_handler(post_id, author_id, request.into(), &self.post_repo).await?;
        Ok(Response::new(Post {
            id: post.id().as_ref().to_string(),
            title: post.title().as_ref().to_string(),
            content: post.content().as_ref().to_string(),
            author_id: post.author_id().as_ref().to_string(),
            created_at: Some(post.created_at().into()),
            updated_at: Some(post.updated_at().into()),
        }))
    }

    #[tracing::instrument(name = "GRPC delete post", skip(self))]
    async fn delete_post(
        &self,
        request: Request<DeletePostCommand>,
    ) -> Result<Response<()>, Status> {
        let author_id = validate_credentials(&request, &self.jwt_service)?;
        let post_id: Identification = Uuid::try_parse(request.get_ref().id.as_str())
            .map_err(|_| Status::invalid_argument("invalid post id"))?
            .into();
        delete_post_handler(post_id, author_id, &self.post_repo).await?;
        Ok(Response::new(()))
    }

    #[tracing::instrument(name = "GRPC get post", skip(self))]
    async fn get_post(&self, request: Request<GetPostCommand>) -> Result<Response<Post>, Status> {
        let post_id: Identification = Uuid::try_parse(request.get_ref().id.as_str())
            .map_err(|_| Status::invalid_argument("invalid post id"))?
            .into();
        let post = get_post_handler(post_id, &self.post_repo).await?;
        Ok(Response::new(Post {
            id: post.id().as_ref().to_string(),
            title: post.title().as_ref().to_string(),
            content: post.content().as_ref().to_string(),
            author_id: post.author_id().as_ref().to_string(),
            created_at: Some(post.created_at().into()),
            updated_at: Some(post.updated_at().into()),
        }))
    }

    #[tracing::instrument(name = "GRPC get post list", skip(self))]
    async fn get_post_list(
        &self,
        request: Request<GetPostListCommand>,
    ) -> Result<Response<GetPostListResult>, Status> {
        let command = request.into_inner();
        let (posts, total) = get_post_list_handler(
            command.limit as usize,
            command.offset as usize,
            &self.post_repo,
        )
        .await?;
        let post = posts
            .iter()
            .map(|post| Post {
                id: post.id().as_ref().to_string(),
                title: post.title().as_ref().to_string(),
                content: post.content().as_ref().to_string(),
                author_id: post.author_id().as_ref().to_string(),
                created_at: Some(post.created_at().into()),
                updated_at: Some(post.updated_at().into()),
            })
            .collect();
        Ok(Response::new(GetPostListResult {
            post,
            total: total as u32,
            offset: command.offset,
            limit: command.limit,
        }))
    }
}

impl From<GetPostListError> for Status {
    fn from(value: GetPostListError) -> Self {
        match value {
            GetPostListError::Unexpected(_) => create_internal_error(),
        }
    }
}

impl From<GetPostError> for Status {
    fn from(value: GetPostError) -> Self {
        match value {
            GetPostError::NotFound => Status::not_found(value.to_string()),
            GetPostError::Unexpected(_) => create_internal_error(),
        }
    }
}

impl From<DeletePostError> for Status {
    fn from(value: DeletePostError) -> Self {
        match value {
            DeletePostError::NotFound => Status::not_found(value.to_string()),
            DeletePostError::NotAllowed => Status::permission_denied(value.to_string()),
            DeletePostError::Unexpected(_) => create_internal_error(),
        }
    }
}

impl From<Request<UpdatePostCommand>> for crate::application::blog::UpdatePostCommand {
    fn from(value: Request<UpdatePostCommand>) -> Self {
        let command = value.into_inner();
        Self::new(command.title, command.content)
    }
}

impl From<UpdatePostError> for Status {
    fn from(value: UpdatePostError) -> Self {
        match value {
            UpdatePostError::NotFound => Status::not_found(value.to_string()),
            UpdatePostError::NotAllowed => Status::permission_denied(value.to_string()),
            UpdatePostError::InvalidTitle(err) => Status::invalid_argument(err.to_string()),
            UpdatePostError::Unexpected(_) => create_internal_error(),
        }
    }
}

impl From<&DateTime> for prost_types::Timestamp {
    fn from(value: &DateTime) -> Self {
        let value = value.as_ref();
        Self {
            seconds: value.timestamp(),
            nanos: value.timestamp_subsec_nanos() as i32,
        }
    }
}

impl From<Request<CreatePostCommand>> for crate::application::blog::CreatePostCommand {
    fn from(value: Request<CreatePostCommand>) -> Self {
        let command = value.into_inner();
        Self::new(command.title, command.content)
    }
}

impl From<CreatePostError> for Status {
    fn from(value: CreatePostError) -> Self {
        match value {
            CreatePostError::InvalidTitle(err) => Status::invalid_argument(err.to_string()),
            CreatePostError::Unexpected(_) => create_internal_error(),
        }
    }
}

impl From<Request<VerifyUserCommand>> for crate::application::auth::VerifyUserCommand {
    fn from(value: Request<VerifyUserCommand>) -> Self {
        let command = value.into_inner();
        Self::new(command.username, SecretString::from(command.password))
    }
}

impl From<VerifyUserError> for Status {
    fn from(value: VerifyUserError) -> Self {
        match value {
            VerifyUserError::UserNotFound => Status::not_found(value.to_string()),
            VerifyUserError::InvalidUserNameOrPassword(err) => Status::invalid_argument(err),
            VerifyUserError::Unexpected(_) => create_internal_error(),
        }
    }
}

impl From<Request<RegisterUserCommand>> for crate::application::auth::RegisterUserCommand {
    fn from(value: Request<RegisterUserCommand>) -> Self {
        let command = value.into_inner();
        Self::new(
            command.username,
            SecretString::from(command.password),
            command.email,
        )
    }
}

impl From<RegisterUserError> for Status {
    fn from(value: RegisterUserError) -> Self {
        match value {
            RegisterUserError::UsernameOrEmailExist => Status::already_exists(value.to_string()),
            RegisterUserError::InvalidUser(err) => Status::invalid_argument(err),
            RegisterUserError::Unexpected(_) => create_internal_error(),
        }
    }
}

fn validate_credentials<T>(
    request: &Request<T>,
    jwt_service: &JwtService,
) -> Result<Identification, Status> {
    match request.metadata().get("authorization") {
        Some(token) => {
            let token = token
                .to_str()
                .map_err(|_| Status::unauthenticated("invalid token format"))?;
            let claims = jwt_service
                .decode_jwt(token)
                .map_err(|_| Status::unauthenticated("invalid token"))?;
            Ok(Identification::from(claims.sub()))
        }
        None => Err(Status::unauthenticated("missing authorization header")),
    }
}

fn create_internal_error() -> Status {
    Status::internal("internal error")
}
