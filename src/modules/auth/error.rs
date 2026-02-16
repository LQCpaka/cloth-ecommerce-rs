// modules/auth/error.rs
use crate::error::AppError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token expired")]
    TokenExpired,

    #[error("Email not verified")]
    EmailNotVerified,

    #[error("Weak password: {reason}")]
    WeakPassword { reason: String },

    #[error("Security error: {0}")]
    SecurityError(String),

    #[error("Account locked")]
    AccountLocked,

    #[error("Too many login attempts")]
    TooManyAttempts,

    #[error("Session expired")]
    SessionExpired,

    #[error("Invalid refresh token")]
    InvalidRefreshToken,

    #[error("Email verification failed")]
    EmailVerificationFailed,

    #[error("Email already verified")]
    EmailAlreadyVerified,

    #[error("Invalid Token")]
    InvalidVerificationToken,

    #[error("Token is expired")]
    VerificationTokenExpired,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

// Convert AuthError -> AppError
impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            // Simple conversions - dùng err.to_string()
            AuthError::UserAlreadyExists => AppError::Conflict(err.to_string()),
            AuthError::UserNotFound => AppError::NotFound(err.to_string()),
            AuthError::InvalidToken | AuthError::TokenExpired | AuthError::InvalidRefreshToken => {
                AppError::Unauthorized(err.to_string())
            }
            AuthError::EmailNotVerified => AppError::Forbidden(err.to_string()),

            // Rate limiting / Security
            AuthError::AccountLocked | AuthError::TooManyAttempts => {
                AppError::Forbidden(err.to_string())
            }

            AuthError::SessionExpired => AppError::Unauthorized(err.to_string()),

            // Email verification issues
            AuthError::EmailVerificationFailed => AppError::BadRequest(err.to_string()),
            AuthError::EmailAlreadyVerified => AppError::Conflict(err.to_string()),

            // Custom handling - che giấu sensitive info
            AuthError::WeakPassword { reason } => {
                tracing::warn!("Weak password rejected: {}", reason);
                AppError::Validation(
                    "Password must be at least 6 characters and contain numbers".to_string(),
                )
            }

            AuthError::SecurityError(msg) => {
                tracing::error!("Auth Security Error: {}", msg);
                AppError::Internal
            }

            AuthError::DatabaseError(e) => {
                tracing::error!("Auth Database Error: {:?}", e);
                AppError::Database(e)
            }

            AuthError::VerificationTokenExpired => {
                AppError::BadRequest("Mã xác thực đã hết hạn".to_string())
            }
            AuthError::InvalidVerificationToken => {
                AppError::BadRequest("Mã xác thực không đúng".to_string())
            }
        }
    }
}
