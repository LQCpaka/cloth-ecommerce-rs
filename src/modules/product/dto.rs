use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::modules::product::model::ProductVariant;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    pub category_id: i32,

    #[validate(length(min = 3, message = "Tên sản phẩm phải có ít nhất 3 ký tự"))]
    pub name: String,

    pub slug: String,

    pub description: Option<String>,

    pub base_price: BigDecimal,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVariantRequest {
    // SKU (Stock Keeping Unit)
    #[validate(length(min = 3, message = "Mã SKU phải có ít nhất 3 ký tự"))]
    pub sku: String,
    pub price_override: Option<BigDecimal>,
    pub stock_quantity: i32,
}

pub struct ProductDetailResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub base_price: BigDecimal,
    //All variant
    pub variants: Vec<ProductVariant>,
}
