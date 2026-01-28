use crate::domain::entities::User;
use crate::domain::value_objects::{Email, UserName};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn exist(&self, username: &UserName, email: &Email) -> Result<bool, anyhow::Error>;
    async fn create(&self, user: &User) -> Result<(), anyhow::Error>;
    async fn get(&self, username: &UserName) -> Result<Option<User>, anyhow::Error>;
}
