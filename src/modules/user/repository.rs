use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::modules::user::model::{AuthProvider, User, UserRole, UserStatus};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
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
                WHERE id = $1
                "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }
}
