use std::sync::Arc;

use uuid::Uuid;

use crate::{error::AppError, shared::services::redis_service::RedisService};

pub struct CartService {
    redis_service: Arc<RedisService>,
}

impl CartService {
    pub fn new(redis_service: Arc<RedisService>) -> Self {
        Self { redis_service }
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
}
