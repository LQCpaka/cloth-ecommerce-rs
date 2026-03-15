use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::AppError, modules::cart::dto::CartItemResponse};

pub struct OrderRepository {
    pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn create_order(
        &self,
        user_id: Uuid,
        address_id: Uuid,                    // 👈 Nhận ID
        address_snapshot: serde_json::Value, // 👈 Nhận Snapshot
        total_amount: &BigDecimal,
        shipping_fee: &BigDecimal,
        notes: Option<&str>,
        cart_items: &[CartItemResponse],
    ) -> Result<Uuid, AppError> {
        let mut tx = self.pool.begin().await.map_err(AppError::Database)?;

        // Nhét đủ các cột theo file sql của vợ
        let order_id: Uuid = sqlx::query_scalar(
            r#"
            INSERT INTO orders (user_id, address_id, shipping_address_snapshot, total_amount, shipping_fee, notes, status, payment_status)
            VALUES ($1, $2, $3, $4, $5, $6, 'pending', 'pending')
            RETURNING id
            "#
        )
        .bind(user_id)
        .bind(address_id)
        .bind(address_snapshot) // SQLx tự parse cái này thành JSONB (quá ngon!)
        .bind(total_amount)
        .bind(shipping_fee)
        .bind(notes)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Lỗi lưu đơn hàng gốc: {:?}", e);
            AppError::Database(e)
        })?;

        // 2. LƯU CHI TIẾT VÀ TRỪ KHO
        for item in cart_items {
            sqlx::query(
                r#"
                    INSERT INTO order_items (order_id, variant_id, quantity, price_at_purchase)
                    VALUES ($1, $2, $3, $4)
                    "#,
            )
            .bind(order_id)
            .bind(item.variant_id)
            .bind(item.quantity)
            .bind(&item.price) // price_at_purchase lấy từ giá lúc bỏ vào giỏ
            .execute(&mut *tx)
            .await
            .map_err(AppError::Database)?;

            // Cập nhật tồn kho trong bảng product_variants
            let result = sqlx::query(
                    "UPDATE product_variants SET stock_quantity = stock_quantity - $1 WHERE id = $2 AND stock_quantity >= $1"
                )
                .bind(item.quantity)
                .bind(item.variant_id)
                .execute(&mut *tx)
                .await?;

            if result.rows_affected() == 0 {
                return Err(AppError::BadRequest(format!(
                    "Sản phẩm '{}' đã hết hàng!",
                    item.product_name
                )));
            }
        }

        tx.commit().await.map_err(AppError::Database)?;
        Ok(order_id)
    }
}
