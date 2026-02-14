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

    #[error("Weak password: {0}")]
    WeakPassword(String),

    #[error("Security error: {0}")]
    SecurityError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

// Convert AuthError -> AppError
impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        match err {
            AuthError::UserAlreadyExists => AppError::Conflict("User already exists".to_string()),
            AuthError::UserNotFound => AppError::NotFound("User not found".to_string()),
            AuthError::InvalidToken => AppError::Unauthorized("Invalid token".to_string()),
            AuthError::TokenExpired => AppError::Unauthorized("Token expired".to_string()),
            AuthError::EmailNotVerified => AppError::Forbidden("Email not verified".to_string()),
            AuthError::WeakPassword(msg) => AppError::Validation(format!("Weak password: {}", msg)),
            AuthError::SecurityError(msg) => {
                tracing::error!("Auth Security Error: {}", msg);
                AppError::Internal
            }
            AuthError::DatabaseError(e) => {
                tracing::error!("Auth Database Error: {:?}", e);
                AppError::Database(e)
            }
        }
    }
}
