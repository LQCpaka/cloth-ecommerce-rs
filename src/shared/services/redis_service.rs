use crate::error::AppError;
use deadpool_redis::{Connection, Pool};
use redis::AsyncTypedCommands;
use std::time::Duration;

pub struct RedisService {
    pool: Pool,
}

impl RedisService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    async fn conn(&self) -> Result<Connection, AppError> {
        self.pool.get().await.map_err(|e| {
            tracing::error!("Redis pool error: {:?}", e);
            AppError::Redis("Không thể kết nối Redis".into())
        })
    }

    pub async fn set<T>(&self, key: &str, value: T, ttl: Duration) -> Result<(), AppError>
    where
        T: redis::ToSingleRedisArg + Send + Sync,
    {
        let mut conn = self.conn().await?;
        let _: () = conn.set_ex(key, value, ttl.as_secs()).await.map_err(|e| {
            tracing::error!("Lỗi khi lưu dữ liệu vào Redis: {:?}", e);
            AppError::Redis("Lỗi ghi dữ liệu vào bộ nhớ đệm".to_string())
        })?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.conn().await?;

        // Take data out, if doesn't have it -> Cache Miss -> return None
        let value: Option<String> = conn.get(key).await.map_err(|e| {
            tracing::error!("Lỗi khi đọc dữ liệu từ Redis: {:?}", e);
            AppError::Redis("Lỗi đọc dữ liệu từ bộ nhớ đệm".to_string())
        })?;

        Ok(value)
    }

    pub async fn delete(&self, key: &str) -> Result<(), AppError> {
        let mut conn = self.conn().await?;

        let _: usize = conn.del(key).await.map_err(|e| {
            tracing::error!("Lỗi khi xóa dữ liệu Redis: {:?}", e);
            AppError::Redis("Lỗi xóa dữ liệu khỏi bộ nhớ đệm".to_string())
        })?;

        Ok(())
    }

    pub async fn rate_limit(
        &self,
        key: &str,
        max_request: i64,
        window: Duration,
    ) -> Result<(), AppError> {
        let mut conn = self.conn().await?;

        // INCR: Increase value to 1. If key doesnt exists, create one
        let count: isize = conn.incr(key, 1).await.map_err(|e| {
            tracing::error!("Lỗi Redis INCR (Rate Limit): {:?}", e);
            AppError::Redis("Lỗi hệ thống chống spam".to_string())
        })?;

        // If this is the first request ? -> Set timeout - CD (TTL)
        if count == 1 {
            let _: bool = conn
                .expire(key, window.as_secs() as i64)
                .await
                .map_err(|e| {
                    tracing::error!("Lỗi Redis EXPIRE (Rate Limit): {:?}", e);
                    AppError::Redis("Lỗi config bộ đếm".to_string())
                })?;
        }

        // Check out if limit is exceeded
        if count > max_request as isize {
            tracing::warn!("Phát hiện spam! Key: {}, Count: {}", key, count);
            return Err(AppError::TooManyRequest(
                "Bạn thao tác quá nhanh. Vui lòng thử lại sau ít phút".to_string(),
            ));
        }
        Ok(())
    }

    // just like HINCRBY key field increment
    pub async fn hincr(&self, key: &str, field: &str, increment: f64) -> Result<f64, AppError> {
        let mut conn = self.conn().await?;

        let new_value: f64 = conn.hincr(key, field, increment).await.map_err(|e| {
            tracing::error!("Lỗi Redis HINCRBY: {:?}", e);
            AppError::Redis("Lỗi cập nhật giỏ hàng".to_string())
        })?;

        Ok(new_value)
    }
}
