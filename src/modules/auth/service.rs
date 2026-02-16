use std::sync::Arc;

use crate::{
    config::Config,
    modules::{
        auth::{
            UserRepository,
            dto::{RegisterRequest, VerifyEmailRequest},
            error::AuthError,
        },
        user::model::User,
    },
    shared::{
        ports::mail::{EmailPayload, MailService},
        services::PasswordService,
    },
};

pub struct AuthService {
    user_repo: Arc<UserRepository>,
    email_service: Arc<dyn MailService>,
    config: Arc<Config>,
}

impl AuthService {
    pub fn new(
        user_repo: Arc<UserRepository>,
        email_service: Arc<dyn MailService>,
        config: Arc<Config>,
    ) -> Self {
        Self {
            user_repo,
            email_service,
            config,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<User, AuthError> {
        if self.user_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AuthError::UserAlreadyExists);
        }

        let hashed_password = PasswordService::hash_password(&req.password)
            .map_err(|e| AuthError::SecurityError(e.to_string()))?;

        let new_user = self
            .user_repo
            .create_user(req.name, req.email, hashed_password)
            .await?;

        // let otp_code = rand::random_range(100000..999999).to_string();
        let verification_token = uuid::Uuid::new_v4().to_string();

        //Save token into DB
        self.user_repo
            .save_otp(new_user.id, &verification_token)
            .await
            .map_err(|e| AuthError::DatabaseError(e))?;

        let email_to_send = new_user.email.clone();
        let name_to_send = new_user.name.clone();
        let email_service_clone = self.email_service.clone();

        let domain_url = self.config.domain_name.clone();
        tokio::spawn(async move {
            let link = format!(
                "http://{}/verify?token={}&email={}",
                domain_url, verification_token, email_to_send
            );

            let html_body = format!(
                "<h3>Chào {}!</h3><p>Vui lòng bấm vào link dưới đây để kích hoạt tài khoản:</p><a href=\"{}\">Kích hoạt ngay</a><p>Link hết hạn sau 15 phút.</p>",
                name_to_send, link
            );

            let payload = EmailPayload {
                to: vec![email_to_send],
                subject: "Kích hoạt tài khoản".to_string(),
                html_body,
                text_body: None,
                cc: None,
                bcc: None,
            };

            if let Err(e) = email_service_clone.send_email(payload).await {
                tracing::error!("Failed to send verification email (OTP DB): {:?}", e)
            }
        });
        Ok(new_user)
    }

    pub async fn verify_account(&self, req: VerifyEmailRequest) -> Result<String, AuthError> {
        // Find if user exists
        let user = self
            .user_repo
            .find_by_email(&req.email)
            .await?
            .ok_or(AuthError::EmailVerificationFailed)?;

        // Check OTP in DB
        self.user_repo
            .find_valid_otp(user.id, &req.token)
            .await?
            .ok_or(AuthError::EmailVerificationFailed)?;

        self.user_repo.active_user(user.id).await?;

        let _ = self.user_repo.delete_user_otps(user.id).await;

        Ok("Kích hoạt tài khoản thành công!".to_string())
    }
}
