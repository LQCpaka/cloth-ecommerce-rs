use rand::Rng;
use std::sync::Arc;

use crate::{
    modules::{
        auth::{UserRepository, dto::RegisterRequest, error::AuthError},
        user::model::User,
    },
    shared::{ports::mail::EmailService, services::PasswordService},
};

pub struct AuthService {
    user_repo: Arc<UserRepository>,
    email_service: Arc<dyn EmailService>,
}

impl AuthService {
    pub fn new(user_repo: Arc<UserRepository>, email_service: Arc<dyn EmailService>) -> Self {
        Self {
            user_repo,
            email_service,
        }
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

        let otp_code = rand::random_range(100000..999999).to_string();

        //Save otp into DB
        self.user_repo
            .save_otp(new_user.id, &otp_code)
            .await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        let email_clone = req.email.clone();
        let email_service_clone = self.email_service.clone();

        tokio::spawn(async move {
            let body = format!(
                "<h1>Xin chào {}!</h1><p>Mã xác thực của bạn là: <b>:{}</b></p><p>Mã xác thực sẽ hết hạn sau 5 phút.</p>",
                req.name, otp_code
            );

            if let Err(e) = email_service_clone
                .send_email(email_clone, "Mã xác thực đăng ký".to_string(), body)
                .await
            {
                tracing::error!("Failed to send OTP email: {}", e)
            }
        });
        Ok(new_user)
    }
}
