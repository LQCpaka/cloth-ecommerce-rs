use sqlx::{Error, PgPool};

pub struct AdminRepository {
    pool: PgPool,
}

impl AdminRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn delete_user(&self, email: String) -> Result<(), Error> {
        let user = sqlx::query!(
            r#"
                DELETE FROM users WHERE email = $1
            "#,
            email
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
