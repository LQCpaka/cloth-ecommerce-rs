use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::{cart::dto::CartItemDb, category::model::Category},
};

pub struct CategoryRepository {
    pool: PgPool,
}

impl CategoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        name: &str,
        slug: &str,
        parent_id: Option<i32>,
    ) -> Result<Category, AppError> {
        let category = sqlx::query_as::<_, Category>(
            r#"
                INSERT INTO categories (name, slug, parent_id)
                VALUES ($1,$2, $3)
                RETURNING *
            "#,
        )
        .bind(name)
        .bind(slug)
        .bind(parent_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Lỗi tạo doanh mục: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(category)
    }

    pub async fn get_all(&self) -> Result<Vec<Category>, AppError> {
        // Should take a look: ORDER BY parent_id NULLS FIRST
        // This helps root categories (those without a parent) always stay at the top, making tree structure easier.
        let categories = sqlx::query_as::<_, Category>(
            "SELECT * FROM categories ORDER BY parent_id ASC NULLS FIRST, id ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Lỗi lấy danh sách danh mục: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(categories)
    }

    pub async fn get_variants_for_cart(
        &self,
        variant_ids: &[Uuid],
    ) -> Result<Vec<CartItemDb>, AppError> {
        // Nếu giỏ hàng trống thì khỏi gọi DB cho đỡ mệt
        if variant_ids.is_empty() {
            return Ok(vec![]);
        }

        let items = sqlx::query_as::<_, CartItemDb>(
                r#"
                SELECT
                    v.id as variant_id,
                    p.name as product_name,
                    v.sku,
                    -- Cực hay: Nếu size này có giá riêng (price_override) thì lấy, ko thì lấy giá gốc
                    COALESCE(v.price_override, p.base_price) as price,
                    -- Lấy 1 tấm hình làm đại diện
                    (SELECT image_url FROM product_images WHERE product_id = p.id LIMIT 1) as thumbnail
                FROM product_variants v
                JOIN products p ON v.product_id = p.id
                -- Tìm tất cả các variant nằm trong danh sách truyền vào
                WHERE v.id = ANY($1)
                "#
            )
            .bind(variant_ids)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Lỗi lấy thông tin giỏ hàng từ DB: {:?}", e);
                AppError::Database(e)
            })?;

        Ok(items)
    }
}
