use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

// 1. Raw Data from DB
#[derive(Debug, FromRow)]
pub struct CartItemDb {
    pub variant_id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub price: BigDecimal, // actual price (checked override_price)
    pub thumbnail: Option<String>,
}

// 2. Each product in bill (DB + Redis)
#[derive(Debug, Serialize)]
pub struct CartItemResponse {
    pub variant_id: Uuid,
    pub product_name: String,
    pub sku: String,
    pub price: BigDecimal,
    pub thumbnail: Option<String>,
    pub quantity: i32,
    pub subtotal: BigDecimal, // = price * quantity
}

// 3. Total Price
#[derive(Debug, Serialize)]
pub struct CartResponse {
    pub items: Vec<CartItemResponse>,
    pub total_price: BigDecimal, // Total price of cart
}

#[derive(Debug, Deserialize, Validate)]
pub struct AddToCartRequest {
    // Mã của món hàng cụ thể (Ví dụ: ID của đôi giày size 42 màu Đen)
    pub variant_id: Uuid,

    // Số lượng muốn nhét vào giỏ
    // Dùng thư viện validator để chặn đứng mấy ông khách cố tình nhập số âm hack hệ thống!
    #[validate(range(min = 1, message = "Số lượng thêm vào giỏ hàng phải từ 1 trở lên!"))]
    pub quantity: i32,
}
