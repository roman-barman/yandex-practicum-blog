use crate::application::contracts::PostRepository;
use crate::domain::entities::Post;
use crate::domain::value_objects::{Content, DateTime, Identification, Title};
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

    #[tracing::instrument(name = "Update post from the DB", skip(self))]
    async fn update(&self, post: &Post) -> Result<(), anyhow::Error> {
        sqlx::query!(
            "UPDATE posts SET title = $1, content = $2, updated_at = $3 WHERE id = $4",
            post.title().as_ref(),
            post.content().as_ref(),
            post.updated_at().as_ref(),
            post.id().as_ref()
        )
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }

    #[tracing::instrument(name = "Get post from the DB", skip(self))]
    async fn get(&self, id: &Identification) -> Result<Option<Post>, anyhow::Error> {
        let record = sqlx::query!("SELECT * FROM posts WHERE id = $1", id.as_ref())
            .fetch_optional(self.pool.as_ref())
            .await?;

        match record {
            None => Ok(None),
            Some(record) => {
                let id = Identification::from(record.id);
                let title = Title::try_from(record.title)?;
                let content = Content::from(record.content);
                let author_id = Identification::from(record.author_id);
                let created_at = DateTime::from(record.created_at);
                let updated_at = DateTime::from(record.updated_at);
                let post = Post::restore(id, title, content, author_id, created_at, updated_at);
                Ok(Some(post))
            }
        }
    }

    #[tracing::instrument(name = "Delete post from the DB", skip(self))]
    async fn delete(&self, id: &Identification) -> Result<(), anyhow::Error> {
        sqlx::query!("DELETE FROM posts WHERE id = $1", id.as_ref())
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    #[tracing::instrument(name = "Get posts list from the DB", skip(self))]
    async fn list(&self, limit: usize, offset: usize) -> Result<(Vec<Post>, usize), anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
            .execute(&mut *tx)
            .await?;

        let count = sqlx::query!("SELECT count(*) as count FROM posts")
            .fetch_one(self.pool.as_ref())
            .await?
            .count
            .unwrap_or(0);

        let records = sqlx::query!(
            "SELECT * FROM posts ORDER BY created_at LIMIT $1 OFFSET $2",
            limit as i64,
            offset as i64
        )
        .fetch_all(self.pool.as_ref())
        .await?;

        tx.commit().await?;

        let mut posts = Vec::with_capacity(records.len());

        for record in records {
            let id = Identification::from(record.id);
            let title = Title::try_from(record.title)?;
            let content = Content::from(record.content);
            let author_id = Identification::from(record.author_id);
            let created_at = DateTime::from(record.created_at);
            let updated_at = DateTime::from(record.updated_at);

            posts.push(Post::restore(
                id, title, content, author_id, created_at, updated_at,
            ));
        }

        Ok((posts, count as usize))
    }
}
