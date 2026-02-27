use bigdecimal::BigDecimal;
use sqlx::PgPool;

use crate::{error::AppError, modules::product::model::Product};

pub struct ProductRepository {
    pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Create new one product
    pub async fn create(
        &self,
        category_id: i32,
        name: &str,
        slug: &str,
        description: Option<String>,
        base_price: BigDecimal,
    ) -> Result<Product, AppError> {
        // Defeault status :  "draft" incase user want to upload imgs, variant
        let product = sqlx::query_as::<_, Product>(
            r#"
                INSERT INTO products (category_id, name, slug, description, base_price)
                VALUES ($1, $2, $3, $4, $5, 'draft)
                RETURNING *
            "#,
        )
        .bind(category_id)
        .bind(name)
        .bind(slug)
        .bind(description)
        .bind(base_price)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Lỗi tạo sản phẩm: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(product)
    }
}
