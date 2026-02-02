use crate::api::grpc::blog::{
    CreatePostCommand, RegisterUserCommand, UpdatePostCommand, VerifyUserCommand,
};
use secrecy::SecretString;
use tonic::Request;

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

impl From<Request<VerifyUserCommand>> for crate::application::auth::VerifyUserCommand {
    fn from(value: Request<VerifyUserCommand>) -> Self {
        let command = value.into_inner();
        Self::new(command.username, SecretString::from(command.password))
    }
}

impl From<Request<CreatePostCommand>> for crate::application::blog::CreatePostCommand {
    fn from(value: Request<CreatePostCommand>) -> Self {
        let command = value.into_inner();
        Self::new(command.title, command.content)
    }
}

impl From<Request<UpdatePostCommand>> for crate::application::blog::UpdatePostCommand {
    fn from(value: Request<UpdatePostCommand>) -> Self {
        let command = value.into_inner();
        Self::new(command.title, command.content)
    }
}
