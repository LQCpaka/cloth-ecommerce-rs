use bigdecimal::BigDecimal;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

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
