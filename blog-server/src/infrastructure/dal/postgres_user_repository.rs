use crate::application::contracts::UserRepository;
use crate::domain::entities::User;
use crate::domain::value_objects::{DateTime, Email, Identification, PasswordHash, UserName};
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use sqlx::PgPool;
use std::sync::Arc;

pub(crate) struct PostgresUserRepository {
    pool: Arc<PgPool>,
}

impl PostgresUserRepository {
    pub(crate) fn new(pool: Arc<PgPool>) -> Self {
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
        .fetch_one(self.pool.as_ref())
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
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }

    #[tracing::instrument(name = "Get user from the DB", skip(self))]
    async fn get(&self, username: &UserName) -> Result<Option<User>, anyhow::Error> {
        let record = sqlx::query!("SELECT * FROM users WHERE username = $1", username.as_ref())
            .fetch_optional(self.pool.as_ref())
            .await?;

        match record {
            None => Ok(None),
            Some(record) => {
                let id = Identification::from(record.id);
                let user_name = UserName::try_from(record.username)?;
                let email = Email::try_from(record.email)?;
                let password_hash = PasswordHash::from(SecretString::from(record.password_hash));
                let created_at = DateTime::from(record.created_at);
                let user = User::restore(id, user_name, email, password_hash, created_at);
                Ok(Some(user))
            }
        }
    }
}
