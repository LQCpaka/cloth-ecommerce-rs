use std::sync::Arc;

use crate::shared::services::redis_service::RedisService;

pub struct CartService {
    redis_service: Arc<RedisService>,
}

impl CartService {
    pub fn new(redis_service: Arc<RedisService>) -> Self {
        Self { redis_service }
    }
}
