use crate::domain::entities::Post;
use crate::domain::value_objects::Identification;
use async_trait::async_trait;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, post: &Post) -> Result<(), anyhow::Error>;
    async fn update(&self, post: &Post) -> Result<(), anyhow::Error>;
    async fn get(&self, id: &Identification) -> Result<Option<Post>, anyhow::Error>;
}
