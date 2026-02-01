use crate::domain::models::product::*;
use bigdecimal::BigDecimal;
use sqlx::PgPool;

pub struct ProductRepository {
    pool: PgPool,
}

impl ProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_product(
        &self,
        category_id: i32,
        name: &str,
        slug: &str,
        description: Option<&str>,
        base_price: BigDecimal,
    ) -> Result<Product, sqlx::Error> {
        let product = sqlx::query_as!(
            Product,
            r#"
            INSERT INTO products (category_id, name, slug, description, base_price)
            VALUES ($1,$2,$3,$4,$5)
            RETURNING id, category_id, name, slug, description, base_price, status, created_at, updated_at
            "#,
            category_id,
            name,
            slug,
            description,
            base_price
        )
        .fetch_one(&self.pool).await?;
        Ok(product)
    }
}
