use crate::api::grpc::blog::blog_service_server::BlogService;
use crate::api::grpc::blog::{
    CreatePostCommand, DeletePostCommand, GetPostCommand, GetPostListCommand, GetPostListResult,
    Post, RegisterUserCommand, UpdatePostCommand, User, VerifyUserCommand, VerifyUserResult,
};
use crate::application::auth::{register_user_handler, verify_user_handler};
use crate::application::blog::{
    create_post_handler, delete_post_handler, get_post_handler, get_post_list_handler,
    update_post_handler,
};
use crate::application::contracts::{PostRepository, UserRepository};
use crate::domain::value_objects::Identification;
use crate::infrastructure::JwtService;
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
        Ok(Response::new(user.into()))
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
            user: Some(user.into()),
        }))
    }

    #[tracing::instrument(name = "GRPC create post", skip(self))]
    async fn create_post(
        &self,
        request: Request<CreatePostCommand>,
    ) -> Result<Response<Post>, Status> {
        let author_id = validate_credentials(&request, &self.jwt_service)?;
        let post = create_post_handler(author_id, request.into(), &self.post_repo).await?;
        Ok(Response::new(post.into()))
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
        Ok(Response::new(post.into()))
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
        Ok(Response::new(post.into()))
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
        let post = posts.into_iter().map(|post| post.into()).collect();
        Ok(Response::new(GetPostListResult {
            post,
            total: total as u32,
            offset: command.offset,
            limit: command.limit,
        }))
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
