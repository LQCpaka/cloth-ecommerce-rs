use bigdecimal::{BigDecimal, Zero};
use std::str::FromStr;
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::{
        cart::dto::{CartItemResponse, CartResponse},
        product::repository::ProductRepository,
    },
    shared::services::redis_service::RedisService,
};

pub struct CartService {
    redis_service: Arc<RedisService>,
    product_repo: Arc<ProductRepository>,
}

impl CartService {
    pub fn new(redis_service: Arc<RedisService>, product_repo: Arc<ProductRepository>) -> Self {
        Self {
            redis_service,
            product_repo,
        }
    }

    pub async fn add_item_to_card(
        &self,
        user_id: Uuid,
        variant_id: Uuid,
        quantity: i32,
    ) -> Result<(), AppError> {
        // Name cart for user
        let cart_key = format!("cart;{}", user_id);

        let field = variant_id.to_string();

        self.redis_service
            .hincr(&cart_key, &field, quantity)
            .await?;

        Ok(())
    }
    pub async fn get_cart(&self, user_id: Uuid) -> Result<CartResponse, AppError> {
        // 1. Lấy danh sách ID và số lượng từ Redis
        let cart_key = format!("cart:{}", user_id);
        let redis_cart = self.redis_service.hgetall(&cart_key).await?;

        // Nếu giỏ trống thì trả về 0 đồng luôn cho lẹ, khỏi gọi DB
        if redis_cart.is_empty() {
            return Ok(CartResponse {
                items: vec![],
                total_price: BigDecimal::zero(),
            });
        }

        // 2. Chuyển đổi Key của Redis (String) thành Uuid để query DB
        let mut variant_ids = Vec::new();
        let mut quantity_map = HashMap::new();

        for (key_str, quantity) in redis_cart {
            if let Ok(id) = Uuid::from_str(&key_str) {
                variant_ids.push(id);
                quantity_map.insert(id, quantity);
            }
        }

        // 3. Nhờ ông thủ kho DB check giá tiền và tên sản phẩm
        let db_items = self
            .product_repo
            .get_variants_for_cart(&variant_ids)
            .await?;

        // 4. Ráp data lại và bấm máy tính tổng tiền
        let mut items = Vec::new();
        let mut total_price = BigDecimal::zero();

        for db_item in db_items {
            // Lấy số lượng từ HashMap Redis ra (mặc định 0 nếu lỗi)
            let quantity = quantity_map.get(&db_item.variant_id).copied().unwrap_or(0);

            if quantity > 0 {
                // Tính tiền từng món: Giá x Số lượng
                let subtotal = &db_item.price * BigDecimal::from(quantity);

                // Cộng dồn vào tổng hóa đơn
                total_price += &subtotal;

                items.push(CartItemResponse {
                    variant_id: db_item.variant_id,
                    product_name: db_item.product_name,
                    sku: db_item.sku,
                    price: db_item.price,
                    thumbnail: db_item.thumbnail,
                    quantity,
                    subtotal,
                });
            }
        }

        Ok(CartResponse { items, total_price })
    }
}
