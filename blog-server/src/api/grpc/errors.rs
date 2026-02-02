use crate::application::auth::{RegisterUserError, VerifyUserError};
use crate::application::blog::{
    CreatePostError, DeletePostError, GetPostError, GetPostListError, UpdatePostError,
};
use tonic::Status;

impl From<CreatePostError> for Status {
    fn from(value: CreatePostError) -> Self {
        match value {
            CreatePostError::InvalidTitle(err) => Status::invalid_argument(err.to_string()),
            CreatePostError::Unexpected(_) => create_internal_error(),
        }
    }
}

impl From<GetPostListError> for Status {
    fn from(value: GetPostListError) -> Self {
        match value {
            GetPostListError::Unexpected(_) => create_internal_error(),
        }
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

impl From<VerifyUserError> for Status {
    fn from(value: VerifyUserError) -> Self {
        match value {
            VerifyUserError::UserNotFound => Status::not_found(value.to_string()),
            VerifyUserError::InvalidUserNameOrPassword(err) => Status::invalid_argument(err),
            VerifyUserError::Unexpected(_) => create_internal_error(),
        }
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

impl From<DeletePostError> for Status {
    fn from(value: DeletePostError) -> Self {
        match value {
            DeletePostError::NotFound => Status::not_found(value.to_string()),
            DeletePostError::NotAllowed => Status::permission_denied(value.to_string()),
            DeletePostError::Unexpected(_) => create_internal_error(),
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

fn create_internal_error() -> Status {
    Status::internal("internal error")
}
