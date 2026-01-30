use crate::application::contracts::PostRepository;
use crate::domain::entities::Post;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

pub(crate) struct PostgresPostRepository {
    pool: Arc<PgPool>,
}

impl PostgresPostRepository {
    pub(crate) fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    #[tracing::instrument(name = "Create post in the DB", skip(self))]
    async fn create(&self, post: &Post) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "INSERT INTO posts VALUES ($1, $2, $3, $4, $5, $6)",
            post.id().as_ref(),
            post.title().as_ref(),
            post.content().as_ref(),
            post.author_id().as_ref(),
            post.created_at().as_ref(),
            post.updated_at().as_ref()
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
}
