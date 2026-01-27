use crate::application::contracts::UserRepository;
use crate::domain::entities::User;
use crate::domain::value_objects::{Email, UserName};
use async_trait::async_trait;
use secrecy::ExposeSecret;
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
    #[tracing::instrument(name = "Check if username or email exists in the DB", skip(self))]
    async fn exist(&self, username: &UserName, email: &Email) -> Result<bool, anyhow::Error> {
        let is_exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = $1 OR email = $2)",
            username.as_ref(),
            email.as_ref(),
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(is_exists.is_some_and(|x| x))
    }

    #[tracing::instrument(name = "Create user in the DB", skip(self))]
    async fn create(&self, user: &User) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
                INSERT INTO users
                VALUES
                ($1, $2, $3, $4, $5)
            "#,
            user.id().as_ref(),
            user.username().as_ref(),
            user.email().as_ref(),
            user.password_hash().as_ref().expose_secret(),
            user.created_at().as_ref(),
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
