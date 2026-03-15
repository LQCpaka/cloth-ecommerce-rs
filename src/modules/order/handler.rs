use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    app_state::AppState,
    error::{AppError, ErrorResponse},
    modules::{
        auth::guard::AuthUser,
        cart::service::CartService,
        order::{dto::CheckoutRequest, repository::OrderRepository, service::OrderService},
        user::repository::UserRepository,
    },
    shared::{services::redis_service::RedisService, utils::flattern_error::flatten_errors},
};

// API: POST /api/v1/orders/checkout
pub async fn checkout(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CheckoutRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Khởi tạo các công cụ và dịch vụ cần thiết
    let redis_service = Arc::new(RedisService::new(state.redis_pool.clone()));

    let cart_service = Arc::new(CartService::new(
        redis_service.clone(),
        state.product_repo.clone(),
    ));

    let order_repo = Arc::new(OrderRepository::new(state.db.clone()));

    // Khởi tạo thêm UserRepository để lấy địa chỉ
    let user_repo = Arc::new(UserRepository::new(state.db.clone()));

    // 2. Lắp ráp thành OrderService hoàn chỉnh
    let order_service = OrderService::new(order_repo, cart_service, redis_service, user_repo);

    // 3. Tiến hành chốt đơn!
    let order_id = order_service.checkout(user.id, payload).await?;

    // 4. Trả về biên lai cho khách
    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "status": "success",
            "message": "Tuyệt vời! Đơn hàng của bạn đã được chốt thành công!",
            "data": { "order_id": order_id }
        })),
    )
        .into_response())
}
