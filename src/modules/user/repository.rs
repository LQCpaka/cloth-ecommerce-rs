use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::user::{
        dto::CreateAddressRequest,
        model::{AuthProvider, User, UserRole, UserStatus},
    },
};

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

    pub async fn create_address(
        &self,
        user_id: Uuid,
        req: CreateAddressRequest,
    ) -> Result<Uuid, AppError> {
        // Tip: If you want the client to set is_default = true later,
        // you can update the old addresses to false here first.
        // For now, just use normal INSERT:

        let address_id: Uuid = sqlx::query_scalar(
                r#"
                INSERT INTO user_addresses
                (user_id, recipient_name, recipient_phone, address_line, ward, district, city, is_default)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id
                "#
            )
            .bind(user_id)
            .bind(req.recipient_name)
            .bind(req.recipient_phone)
            .bind(req.address_line)
            .bind(req.ward)
            .bind(req.district)
            .bind(req.city)
            .bind(req.is_default.unwrap_or(false)) // Mặc định false nếu FE không gửi
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Lỗi lưu địa chỉ: {:?}", e);
                AppError::Database(e)
            })?;

        Ok(address_id)
    }
}
