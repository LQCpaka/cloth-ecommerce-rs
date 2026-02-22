use std::sync::Arc;

use crate::{
    config::Config,
    modules::{
        auth::{
            AuthRepository,
            dto::{LoginRequest, RegisterRequest, ResendVerifyEmailRequest, VerifyEmailRequest},
            error::AuthError,
        },
        user::model::{User, UserStatus},
    },
    shared::{
        ports::mail::MailService,
        services::{
            PasswordService,
            email_sample::{EmailConfig, send_register_verification, send_resend_verification},
            jwt::TokenService,
        },
    },
};

pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct AuthService {
    auth_repo: Arc<AuthRepository>,
    email_service: Arc<dyn MailService>,
    token_service: Arc<TokenService>,
    config: Arc<Config>,
}

impl AuthService {
    pub fn new(
        auth_repo: Arc<AuthRepository>,
        email_service: Arc<dyn MailService>,
        token_service: Arc<TokenService>,
        config: Arc<Config>,
    ) -> Self {
        Self {
            auth_repo,
            email_service,
            token_service,
            config,
        }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<User, AuthError> {
        if self.auth_repo.find_by_email(&req.email).await?.is_some() {
            return Err(AuthError::UserAlreadyExists);
        }

        let hashed_password = PasswordService::hash_password(&req.password)
            .map_err(|e| AuthError::SecurityError(e.to_string()))?;

        let new_user = self
            .auth_repo
            .create_user(req.name, req.email, hashed_password)
            .await?;

        // let otp_code = rand::random_range(100000..999999).to_string();
        let verification_token = uuid::Uuid::new_v4().to_string();

        //Save token into DB
        self.auth_repo
            .save_otp(new_user.id, &verification_token)
            .await
            .map_err(|e| AuthError::DatabaseError(e))?;

        let email_to_send = new_user.email.clone();
        let name_to_send = new_user.name.clone();
        let email_service_clone = self.email_service.clone();

        let domain_url = self.config.domain_name.clone();

        send_register_verification(EmailConfig {
            email_to_send: email_to_send.clone(),
            name_to_send: name_to_send.clone(),
            email_service: email_service_clone.clone(),
            domain_url: domain_url.clone(),
            verification_token: verification_token,
        });

        Ok(new_user)
    }

    pub async fn verify_account(&self, req: VerifyEmailRequest) -> Result<String, AuthError> {
        // Find if user exists
        let user = self
            .auth_repo
            .find_by_email(&req.email)
            .await?
            .ok_or(AuthError::EmailVerificationFailed)?;

        if user.status == UserStatus::Active {
            return Ok("Tài khoản này đã được kích hoạt rồi".to_string());
        }
        let otp_record = self
            .auth_repo
            .find_otp_record(user.id, &req.token)
            .await?
            .ok_or(AuthError::InvalidVerificationToken)?;

        if otp_record.expires_at < chrono::Utc::now() {
            let _ = self.auth_repo.delete_user_otps(user.id).await?;
            return Err(AuthError::VerificationTokenExpired);
        }

        // If still valid and not expired -> Active account
        self.auth_repo
            .active_user(user.id)
            .await
            .map_err(|e| AuthError::DatabaseError(e))?;

        // Clean verification token
        let _ = self.auth_repo.delete_user_otps(user.id).await;

        Ok("Kích hoạt tài khoản thành công!".to_string())
    }

    pub async fn resend_verification_email(
        &self,
        req: ResendVerifyEmailRequest,
    ) -> Result<String, AuthError> {
        let user = self
            .auth_repo
            .find_by_email(&req.email)
            .await?
            .ok_or(AuthError::UserNotFound)?;

        if user.status == UserStatus::Active {
            return Ok("Tài khoản này vốn đã được kích hoạt".to_string());
        }

        let _ = self.auth_repo.delete_user_otps(user.id).await;
        let verification_token = uuid::Uuid::new_v4().to_string();

        self.auth_repo
            .save_otp(user.id, &verification_token)
            .await
            .map_err(|e| AuthError::DatabaseError(e))?;

        send_resend_verification(EmailConfig {
            email_to_send: user.email.clone(),
            name_to_send: user.name.clone(),
            email_service: self.email_service.clone(),
            domain_url: self.config.domain_name.clone(),
            verification_token: verification_token,
        });

        Ok("Đã gửi lại email xác thực tài khoản về tài khoản, vui lòng kiểm tra lại.".to_string())
    }

    pub async fn login(
        &self,
        req: LoginRequest,
        user_agent: String,
    ) -> Result<TokenPair, AuthError> {
        // 1. Find user
        let user = self
            .auth_repo
            .find_by_email(&req.email)
            .await?
            .ok_or(AuthError::AuthorizationFailed)?;

        let db_passwordhash = user
            .password_hash
            .as_ref()
            .ok_or(AuthError::AuthorizationFailed)?;

        // 2. Password validate
        let is_valid = PasswordService::verify_password(&req.password, db_passwordhash)
            .map_err(|_| AuthError::AuthorizationFailed)?;

        if !is_valid {
            return Err(AuthError::AuthorizationFailed);
        }

        if user.status == UserStatus::Unverified {
            return Err(AuthError::EmailNotVerified);
        }
        if user.status == UserStatus::Banned {
            return Err(AuthError::AccountLocked);
        }

        // 3. Generate Access Token
        let access_token = self
            .token_service
            .generate_access_token(user.id, format!("{:?}", user.role), user.email)
            .map_err(|e| AuthError::SecurityError(e.to_string()))?;

        // 4. Generate Refresh Token
        let refresh_token = self.token_service.generate_refresh_token();

        // 5. Lưu Session
        self.auth_repo
            .create_session(
                user.id,
                &refresh_token,
                &user_agent,
                self.config.refresh_token_expired_in,
            )
            .await
            .map_err(AuthError::DatabaseError)?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}
