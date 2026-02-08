use crate::model::{AuthProvider, User, UserSession, UserStatus};
use anyhow::Ok;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    // Create Func
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Create new user - Register
    pub async fn create_user(
        &self,
        name: String,
        email: String,
        password_hash: String,
    ) -> Result<User, Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password_hash, role, status, provider, provider_id)
            VALUES ($1, $2, $3, 'user', 'unverified', 'local', NULL)
            RETURNING *
            "#,
            name,
            email,
            password_hash
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?;

        Ok(user)
    }
}
