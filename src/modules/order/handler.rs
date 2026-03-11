use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    app_state::AppState,
    error::AppError,
    modules::{
        auth::guard::AuthUser,
        cart::service::CartService,
        order::{dto::CheckoutRequest, service::OrderService},
    },
    shared::services::redis_service::RedisService,
};

// src/modules/order/handler.rs
pub async fn checkout(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CheckoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload
        .validate()
        .map_err(|_| AppError::BadRequest("Thông tin giao hàng không hợp lệ".into()))?;

    // Khởi tạo các service cần thiết
    let cart_service = Arc::new(CartService::new(
        Arc::new(RedisService::new(state.redis_pool.clone())),
        state.product_repo.clone(),
    ));

    let order_service = OrderService::new(
        state.order_repo.clone(),
        cart_service,
        Arc::new(RedisService::new(state.redis_pool.clone())),
    );

    let order_id = order_service.checkout(user.id, payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "status": "success",
            "message": "Đặt hàng thành công! Shipper đang chuẩn bị lên đường.",
            "data": { "order_id": order_id }
        })),
    )
        .into_response())
}
