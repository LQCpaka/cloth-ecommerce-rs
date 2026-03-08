use std::sync::Arc;

use uuid::Uuid;

use crate::{
    error::AppError,
    modules::product::{
        dto::{
            CreateProductRequest, CreateVariantRequest, ProductDetailResponse,
            ProductListItemResponse, ProductListQuery,
        },
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

    pub async fn get_products(
        &self,
        query: ProductListQuery,
    ) -> Result<Vec<ProductListItemResponse>, AppError> {
        // take default as page 1, each page take 10
        let page = query.page.unwrap_or(1).max(1); // prevent customer set neg number (e.g: -8)
        let limit = query.limit.unwrap_or(10).clamp(1, 100);

        self.repo.get_products(page, limit).await
    }
}
