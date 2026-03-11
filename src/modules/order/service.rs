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
        // 1. Lấy giỏ hàng hiện tại để biết khách định mua gì
        let cart = self.cart_service.get_cart(user_id).await?;
        if cart.items.is_empty() {
            return Err(AppError::BadRequest(
                "Giỏ hàng trống rỗng, mua gì mà mua!".into(),
            ));
        }

        // 2. Lưu vào Database (Transaction đã lo hết ở Repo)
        let shipping_fee = BigDecimal::from(0); // Tạm thời free ship cho vợ yêu
        let order_id = self
            .order_repo
            .create_order(
                user_id,
                &cart.total_price,
                &shipping_fee,
                req.note.as_ref(),
                &cart.items,
            )
            .await?;

        // 3. Mua xong rồi thì xóa giỏ hàng trong Redis cho sạch sẽ
        let cart_key = format!("cart:{}", user_id);
        let _ = self.redis_service.delete(&cart_key).await;

        Ok(order_id)
    }
}
