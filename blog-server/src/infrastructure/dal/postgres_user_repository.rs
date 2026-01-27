use crate::application::contracts::UserRepository;
use crate::domain::value_objects::UserName;
use async_trait::async_trait;
use sqlx::PgPool;

pub(crate) struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    #[tracing::instrument(name = "Check if user exists in the DB", skip(self))]
    async fn exist(&self, username: &UserName) -> Result<bool, anyhow::Error> {
        let is_exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1)",
            username.as_ref()
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(is_exists.is_some_and(|x| x))
    }
}
