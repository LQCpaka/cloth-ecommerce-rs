use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::product::{
        dto::{ProductDetailResponse, ProductListItemResponse},
        model::{Product, ProductVariant},
    },
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

    pub async fn get_products(
        &self,
        page: i64,
        limit: i64,
    ) -> Result<Vec<ProductListItemResponse>, AppError> {
        let offset = (page - 1) * limit;

        let products = sqlx::query_as::<_, ProductListItemResponse>(
                r#"
                SELECT
                    p.id,
                    p.name,
                    p.slug,
                    p.base_price,
                    (SELECT image_url FROM product_images where product_id = p.id LIMIT 1) as thumbnail
                FROM products p
                ORDER BY p.created_at DESC
                LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Lỗi lấy danh sách sản phẩm: {:?}",e);
            AppError::Database(e)
        })?;
        Ok(products)
    }
}
