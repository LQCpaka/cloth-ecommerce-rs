use sqlx::{Error, PgPool};
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::{
        product::dto::ProductDetailResponse,
        user::model::{AuthProvider, User, UserRole, UserStatus},
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

    pub async fn get_product_detail(
        &self,
        product_id: Uuid,
    ) -> Result<ProductDetailResponse, AppError> {
        let product = sqlx::query_as::<_, ProductDetailResponse>(
            r#"
                SELECT
                    p.id,
                    p.name,
                    p.slug,
                    p.description,
                    p.base_price,
                    p.status,
                    COALESCE(
                        (SELECT array_agg(image_url) FROM product_images WHERE product_id = p.id),
                        ARRAY[]::text[]
                    ) as images,
                    COALESCE(
                        (SELECT jsonb_agg(
                            jsonb_build_object(
                                'id', v.id,
                                'product_id', v.product_id,
                                'sku', v.sku,
                                'price_override', v.price_override,
                                'stock_quantity', v.stock_quantity,
                                'created_at', v.created_at,
                                'updated_at', v.updated_at
                            )
                        ) FROM product_variants v WHERE v.product_id = p.id),
                        '[]'::jsonb
                    ) as variants
                FROM products p
                WHERE p.id = $1
                "#,
        )
        .bind(product_id)
        .fetch_optional(&self.pool) // Dùng fetch_optional vì có thể khách gõ sai ID
        .await
        .map_err(|e| {
            tracing::error!("Lỗi lấy chi tiết sản phẩm: {:?}", e);
            AppError::Database(e)
        })?
        // Nếu DB trả về None (không tìm thấy), mình quăng lỗi 404 luôn
        .ok_or_else(|| AppError::NotFound("Không tìm thấy sản phẩm này!".to_string()))?;

        Ok(product)
    }
}
