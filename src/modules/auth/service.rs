use std::sync::Arc;

use crate::{
    modules::{
        auth::{UserRepository, dto::RegisterRequest, error::AuthError},
        user::model::User,
    },
    shared::services::PasswordService,
};

pub struct AuthService {
    user_repo: Arc<UserRepository>,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<User, AuthError> {
        if let Some(_) = self.user_repo.find_by_email(&req.email).await? {
            return Err(AuthError::EmailAlreadyExists);
        }

        let hashed_password = PasswordService::hash_password(&req.password)
            .map_err(|e| AuthError::SecurityError(e.to_string()))?;

        let new_user = self
            .user_repo
            .create_user(req.name, req.email, hashed_password)
            .await?;

        Ok(new_user)
    }
}
