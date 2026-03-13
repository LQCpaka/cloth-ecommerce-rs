use std::sync::Arc;

use crate::{
    error::AppError,
    modules::admin::{dto::DeleteAccountRequest, repository::AdminRepository},
};

pub struct AdminService {
    admin_repo: Arc<AdminRepository>,
}

impl AdminService {
    pub fn new(admin_repo: Arc<AdminRepository>) -> Self {
        Self { admin_repo }
    }

    pub async fn delete_user(&self, req: DeleteAccountRequest) -> Result<(), AppError> {
        if self.admin_repo.delete_user(&req.email).await? {
            return Err(AppError::Conflict("User did not exists!".into()));
        }

        Ok(())
    }
}
