use crate::api::grpc::blog::{Post, User};
use crate::domain::value_objects::DateTime;

impl From<crate::domain::entities::User> for User {
    fn from(user: crate::domain::entities::User) -> Self {
        User {
            id: user.id().as_ref().to_string(),
            username: user.username().as_ref().to_string(),
            email: user.email().as_ref().to_string(),
        }
    }
}

impl From<crate::domain::entities::Post> for Post {
    fn from(post: crate::domain::entities::Post) -> Self {
        Post {
            id: post.id().as_ref().to_string(),
            title: post.title().as_ref().to_string(),
            content: post.content().as_ref().to_string(),
            author_id: post.author_id().as_ref().to_string(),
            created_at: Some(post.created_at().into()),
            updated_at: Some(post.updated_at().into()),
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
