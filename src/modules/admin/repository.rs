use sqlx::PgPool;

pub struct AdminRepository {
    pool: PgPool,
}

impl AdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn delete_user(&self, email: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM users WHERE email = $1", email)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn ban_user(&self, email: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE users
            SET status = 'banned'
            WHERE email = $1
            "#,
            email
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
