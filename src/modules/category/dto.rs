use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryTreeResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub children: Vec<CategoryTreeResponse>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, message = "Tên không được để trống"))]
    pub name: String,
    #[validate(length(min = 1, message = "Slug không được để trống"))]
    pub slug: String,
    pub parent_id: Option<i32>,
}
