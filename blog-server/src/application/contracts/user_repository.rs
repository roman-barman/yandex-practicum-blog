use crate::domain::value_objects::{Email, UserName};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn exist(&self, username: &UserName, email: &Email) -> Result<bool, anyhow::Error>;
}
