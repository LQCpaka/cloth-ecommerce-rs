use std::sync::Arc;

use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::{
        cart::service::CartService,
        order::{dto::CheckoutRequest, repository::OrderRepository},
    },
    shared::services::redis_service::RedisService,
};

// src/modules/order/service.rs
pub struct OrderService {
    order_repo: Arc<OrderRepository>,
    cart_service: Arc<CartService>,
    redis_service: Arc<RedisService>,
}

impl OrderService {
    pub fn new(
        order_repo: Arc<OrderRepository>,
        cart_service: Arc<CartService>,
        redis_service: Arc<RedisService>,
    ) -> Self {
        Self {
            order_repo,
            cart_service,
            redis_service,
        }
    }
    pub async fn checkout(&self, user_id: Uuid, req: CheckoutRequest) -> Result<Uuid, AppError> {
        // 1. Lấy thông tin địa chỉ từ DB (giả sử vợ có AddressRepo)
        let address = self
            .address_repo
            .find_by_id(req.address_id, user_id)
            .await?
            .ok_or(AppError::NotFound("Địa chỉ không tồn tại!".into()))?;

        // 2. TẠO SNAPSHOT: Chuyển object địa chỉ thành JSON để lưu vào bảng orders
        // Sau này khách có đổi địa chỉ trong Profile thì Snapshot này vẫn giữ nguyên thông tin lúc mua
        let address_snapshot = serde_json::json!({
            "recipient_name": address.recipient_name,
            "recipient_phone": address.recipient_phone,
            "address_line": address.address_line,
            "ward": address.ward,
            "district": address.district,
            "city": address.city
        });

        // 3. Lấy giỏ hàng như cũ...
        let cart = self.cart_service.get_cart(user_id).await?;

        // 4. Truyền thêm address_id và snapshot vào Repo
        let order_id = self
            .order_repo
            .create_order(
                user_id,
                req.address_id,   // 👈 Thêm bạn này
                address_snapshot, // 👈 Thêm bạn này (kiểu serde_json::Value)
                &cart.total_price,
                // ... các tham số khác
            )
            .await?;

        // 3. Mua xong rồi thì xóa giỏ hàng trong Redis cho sạch sẽ
        let cart_key = format!("cart:{}", user_id);
        let _ = self.redis_service.delete(&cart_key).await;

        Ok(order_id)
    }
}
