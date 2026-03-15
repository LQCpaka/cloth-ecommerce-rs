use std::sync::Arc;

use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::{
    error::AppError,
    modules::{
        cart::service::CartService,
        order::{dto::CheckoutRequest, repository::OrderRepository},
        user::{repository::UserRepository, service::UserService},
    },
    shared::services::redis_service::RedisService,
};

// src/modules/order/service.rs
pub struct OrderService {
    order_repo: Arc<OrderRepository>,
    cart_service: Arc<CartService>,
    redis_service: Arc<RedisService>,
    user_repo: Arc<UserRepository>,
}

impl OrderService {
    pub fn new(
        order_repo: Arc<OrderRepository>,
        cart_service: Arc<CartService>,
        redis_service: Arc<RedisService>,
        user_repo: Arc<UserRepository>,
    ) -> Self {
        Self {
            order_repo,
            cart_service,
            redis_service,
            user_repo,
        }
    }
    pub async fn checkout(&self, user_id: Uuid, req: CheckoutRequest) -> Result<Uuid, AppError> {
        // 1. CHỤP ẢNH ĐỊA CHỈ: Lấy Snapshot từ DB
        let address_snapshot = self
            .user_repo
            .get_address_snapshot(req.address_id, user_id)
            .await?
            .ok_or_else(|| {
                AppError::BadRequest("Địa chỉ không tồn tại hoặc không thuộc về bạn!".into())
            })?;

        // 2. LẤY GIỎ HÀNG
        let cart = self.cart_service.get_cart(user_id).await?;
        if cart.items.is_empty() {
            return Err(AppError::BadRequest("Giỏ hàng trống rỗng!".into()));
        }

        // 3. TẠO ĐƠN HÀNG (Truyền thêm address_id và snapshot)
        let shipping_fee = BigDecimal::from(0);
        let order_id = self
            .order_repo
            .create_order(
                user_id,
                req.address_id,
                address_snapshot, // 👈 Nhét cục JSON vào đây
                &cart.total_price,
                &shipping_fee,
                req.note.as_deref(),
                &cart.items,
            )
            .await?;

        // 4. XÓA GIỎ HÀNG SAU KHI MUA THÀNH CÔNG
        let cart_key = format!("cart:{}", user_id);
        let _ = self.redis_service.delete(&cart_key).await;

        Ok(order_id)
    }
}
