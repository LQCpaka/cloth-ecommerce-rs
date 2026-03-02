use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::product::model::{Product, ProductVariant},
};

pub struct ProductRepository {
    pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Create New (base product)
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
                INSERT INTO products (category_id, name, slug, description, base_price, status)
                VALUES ($1, $2, $3, $4, $5, 'draft')
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

    pub async fn create_variant(
        &self,
        product_id: Uuid,
        sku: &str,
        price_override: Option<BigDecimal>,
        stock_quantity: i32,
    ) -> Result<ProductVariant, AppError> {
        let variant = sqlx::query_as::<_, ProductVariant>(
            r#"
                INSERT INTO product_variants (product_id, sku, price_override, stock_quantity)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
        )
        .bind(product_id)
        .bind(sku)
        .bind(price_override)
        .bind(stock_quantity)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            //If seller type input same SKU, DB will return an error, something like that
            tracing::error!("Lỗi tạo variant (nhập kho): {:?}", e);
            AppError::Database(e)
        })?;

        Ok(variant)
    }

    pub async fn add_product_image(
        &self,
        product_id: Uuid,
        image_url: &str,
    ) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO product_images (product_id, image_url)
            VALUES ($1, $2)
            "#,
        )
        .bind(product_id)
        .bind(image_url)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Lỗi lưu link ảnh sản phẩm vào DB: {:?}", e);
            AppError::Database(e)
        })?;

        Ok(())
    }
}
