use std::{collections::HashMap, sync::Arc};

use crate::{
    error::AppError,
    modules::category::{
        dto::{CategoryTreeResponse, CreateCategoryRequest},
        model::Category,
        repository::CategoryRepository,
    },
};

pub struct CategoryService {
    repo: Arc<CategoryRepository>,
}

impl CategoryService {
    pub fn new(repo: Arc<CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_category_tree(&self) -> Result<Vec<CategoryTreeResponse>, AppError> {
        let categories = self.repo.get_all().await?;

        let mut children_map: HashMap<Option<i32>, Vec<Category>> = HashMap::new();
        for cat in categories {
            children_map
                .entry(cat.parent_id)
                .or_insert_with(Vec::new)
                .push(cat);
        }

        let tree = Self::build_tree(None, &children_map);
        Ok(tree)
    }

    pub async fn create_category(&self, req: CreateCategoryRequest) -> Result<Category, AppError> {
        // Chỗ này sau này có thể làm thêm logic check xem 'slug' đã tồn tại chưa nha.
        self.repo.create(&req.name, &req.slug, req.parent_id).await
    }

    // Recursive helper
    fn build_tree(
        parent_id: Option<i32>,
        map: &HashMap<Option<i32>, Vec<Category>>,
    ) -> Vec<CategoryTreeResponse> {
        let mut result = Vec::new();

        //Checkin parent_id if there is any child
        if let Some(children) = map.get(&parent_id) {
            for child in children {
                // on every single child -> checkin if there is any other child of that child
                let node = CategoryTreeResponse {
                    id: child.id,
                    name: child.name.clone(),
                    slug: child.slug.clone(),
                    //callback
                    children: Self::build_tree(Some(child.id), map),
                };
                result.push(node);
            }
        }
        result
    }
}
