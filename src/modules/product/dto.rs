use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryTreeResponse {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub children: Vec<CategoryTreeResponse>,
}
