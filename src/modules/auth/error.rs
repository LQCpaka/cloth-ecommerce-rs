use crate::error::AppError;
use thiserror::Error;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("User not found")]
    UserNotFound,

    #[error("Login failed")]
    AuthorizationFailed,

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

    #[error("Invalid verification token")]
    InvalidVerificationToken,

    #[error("Verification token expired")]
    VerificationTokenExpired,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        let msg = err.to_string(); // lấy trước khi move
        match err {
            // User enumeration protection:
            // Gộp UserNotFound + AuthorizationFailed thành một message chung
            // tránh attacker biết email nào đã đăng ký
            AuthError::UserNotFound | AuthError::AuthorizationFailed => {
                AppError::InvalidCredentials("Invalid email or password".to_string())
            }

            // Registration
            AuthError::UserAlreadyExists => AppError::Conflict(msg),

            // Token errors
            AuthError::InvalidToken | AuthError::TokenExpired | AuthError::InvalidRefreshToken => {
                AppError::Unauthorized(msg)
            }

            // Session
            AuthError::SessionExpired => AppError::Unauthorized(msg),

            // Access control
            AuthError::EmailNotVerified => AppError::Forbidden(msg),

            // Rate limiting / Security
            AuthError::AccountLocked | AuthError::TooManyAttempts => AppError::Forbidden(msg),

            // Email verification
            AuthError::EmailVerificationFailed => AppError::BadRequest(msg),
            AuthError::EmailAlreadyVerified => AppError::Conflict(msg),
            AuthError::VerificationTokenExpired => {
                AppError::BadRequest("Verification code has expired".to_string())
            }
            AuthError::InvalidVerificationToken => {
                AppError::BadRequest("Invalid verification code".to_string())
            }

            // Validation
            AuthError::WeakPassword { reason } => {
                tracing::warn!("Weak password rejected: {}", reason);
                AppError::Validation(
                    "Password must be at least 8 characters and contain letters and numbers"
                        .to_string(),
                )
            }

            // Security - log đầy đủ nhưng không trả details ra ngoài
            AuthError::SecurityError(inner) => {
                tracing::error!("Auth security error: {}", inner);
                AppError::Internal
            }

            // Database - log đầy đủ, AppError::Database sẽ ẩn details khi trả về client
            AuthError::DatabaseError(e) => {
                tracing::error!("Auth database error: {:?}", e);
                AppError::Database(e)
            }
        }
    }
}
