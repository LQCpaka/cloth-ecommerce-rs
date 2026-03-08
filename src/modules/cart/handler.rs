use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    app_state::AppState,
    error::AppError,
    modules::{
        auth::guard::AuthUser,
        cart::{dto::AddToCartRequest, service::CartService},
    },
    shared::services::redis_service::RedisService,
};

// API: GET /api/v1/cart
pub async fn get_cart(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    // Gọi Service truyền cả Redis và Repo vào
    let cart_service = CartService::new(
        Arc::new(RedisService::new(state.redis_pool.clone())),
        state.product_repo.clone(),
    );

    // Lấy hóa đơn
    let cart_response = cart_service.get_cart(user.id).await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "data": cart_response
        })),
    )
        .into_response())
}

// API: POST /api/v1/cart
pub async fn add_to_cart(
    State(state): State<AppState>,
    user: AuthUser, // 👈 Bắt buộc phải có thẻ VIP (đăng nhập) mới được lấy xe đẩy
    Json(payload): Json<AddToCartRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Kiểm tra xem khách có nhập bậy bạ không (ví dụ: số lượng âm)
    if let Err(_e) = payload.validate() {
        return Err(AppError::BadRequest(
            "Số lượng không hợp lệ! Vui lòng kiểm tra lại.".to_string(),
        ));
    }

    // 2. Khởi tạo CartService với đầy đủ 2 món vũ khí (Redis + Repo) giống hệt get_cart
    let cart_service = CartService::new(
        Arc::new(RedisService::new(state.redis_pool.clone())),
        state.product_repo.clone(), // 👈 Đưa chìa khóa kho cho nó
    );

    // 3. Kêu Service ném hàng vào tủ Redis
    cart_service
        .add_item_to_card(user.id, payload.variant_id, payload.quantity)
        .await?;

    // 4. Báo cáo thành công cho Frontend
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": "Đã thêm sản phẩm vào giỏ hàng!",
        })),
    )
        .into_response())
}
