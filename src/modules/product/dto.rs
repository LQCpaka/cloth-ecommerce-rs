use bigdecimal::BigDecimal;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    pub product_id: i32,

    #[validate(length(min = 3, message = "Tên sản phẩm phải có ít nhất 3 ký tự"))]
    pub name: String,

    pub slug: String,

    pub description: Option<String>,

    pub base_price: BigDecimal,
}
