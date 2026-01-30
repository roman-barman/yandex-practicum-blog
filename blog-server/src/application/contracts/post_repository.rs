use crate::domain::entities::Post;
use async_trait::async_trait;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, post: &Post) -> Result<(), anyhow::Error>;
}
