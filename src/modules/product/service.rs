use std::sync::Arc;

use crate::{
    error::AppError,
    modules::product::{dto::CreateProductRequest, model::Product, repository::ProductRepository},
};

pub struct ProductService {
    repo: Arc<ProductRepository>,
}

impl ProductService {
    pub fn new(repo: Arc<ProductRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_product(&self, req: CreateProductRequest) -> Result<Product, AppError> {
        self.repo
            .create(
                req.category_id,
                &req.name,
                &req.slug,
                req.description,
                req.base_price,
            )
            .await
    }
}
