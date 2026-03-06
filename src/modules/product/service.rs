use std::sync::Arc;

use uuid::Uuid;

use crate::{
    error::AppError,
    modules::product::{
        dto::{CreateProductRequest, CreateVariantRequest, ProductDetailResponse},
        model::{Product, ProductVariant},
        repository::ProductRepository,
    },
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

    pub async fn create_variant(
        &self,
        product_id: uuid::Uuid,
        req: CreateVariantRequest,
    ) -> Result<ProductVariant, AppError> {
        self.repo
            .create_variant(product_id, &req.sku, req.price_override, req.stock_quantity)
            .await
    }

    pub async fn get_product_detail(
        &self,
        product_id: Uuid,
    ) -> Result<ProductDetailResponse, AppError> {
        self.repo.get_product_detail(product_id).await
    }
}
