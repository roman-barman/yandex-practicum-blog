use crate::domain::value_objects::UserName;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn exist(&self, username: &UserName) -> Result<bool, anyhow::Error>;
}
