use crate::modules::user::model::{AuthProvider, User, UserRole, UserStatus};
use sqlx::{Error, PgPool};

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
                RETURNING
                    id,
                    email,
                    password_hash,
                    name,
                    avatar_url,
                    description,
                    role as "role: UserRole",
                    status as "status: UserStatus",
                    provider as "provider: AuthProvider",
                    provider_id,
                    created_at,
                    updated_at
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
        let user = sqlx::query_as!(
            User,
            r#"
                SELECT
                    id,
                    email,
                    password_hash,
                    name,
                    avatar_url,
                    description,
                    role as "role: UserRole",
                    status as "status: UserStatus",
                    provider as "provider: AuthProvider",
                    provider_id,
                    created_at,
                    updated_at
                FROM users
                WHERE email = $1
                "#,
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn save_otp(&self, user_id: uuid::Uuid, code: &str) -> Result<(), Error> {
        sqlx::query!(
            // Expires in 5 min
            r#"
                INSERT INTO user_otps (user_id, code, expires_at)
                VALUES ($1,$2, now() + interval '5 minutes')
            "#,
            user_id,
            code
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn find_valid_otp(
        &self,
        user_id: uuid::Uuid,
        code: &str,
    ) -> Result<Option<uuid::Uuid>, Error> {
        let result = sqlx::query!(
            r#"
                SELECT id FROM user_otps
                WHERE user_id = $1
                AND code = $2
                AND expires_at > NOW()
                LIMIT 1
            "#,
            user_id,
            code
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|r| r.id))
    }

    pub async fn active_user(&self, user_id: uuid::Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
                UPDATE users
                SET status = 'active'::user_status_type, updated_at = NOW()
                WHERE id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_user_otps(&self, user_id: uuid::Uuid) -> Result<(), Error> {
        sqlx::query!(r#"DELETE FROM user_otps WHERE user_id = $1"#, user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
