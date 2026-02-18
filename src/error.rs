use std::collections::HashMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

// Env config Error
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid number for {0}: {1}")]
    InvalidNumber(String, String),

    #[error("Environment error: {0}")]
    EnvError(#[from] dotenvy::Error),

    #[error("Invalid email format: {0}")]
    InvalidEmailFormat(String),
}

// Application-wide error type
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum AppError {
    // Configuration errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    // Database errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    // Auth errors
    #[error("Authentication failed: {0}")]
    Unauthorized(String),

    #[error("Access forbidden: {0}")]
    Forbidden(String),

    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Validation errors")]
    ValidationFields(HashMap<String, Vec<String>>),

    #[error("Bad request: {0}")]
    BadRequest(String),

    // Resource errors
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Resource already exists: {0}")]
    Conflict(String),

    // External service errors
    #[error("Email service error: {0}")]
    EmailService(String),

    #[error("Storage service error: {0}")]
    StorageService(String),

    #[error("Redis error: {0}")]
    Redis(String),

    // Business logic errors
    #[error("Business logic error: {0}")]
    BusinessLogic(String),

    // Generic errors
    #[error("Internal server error")]
    Internal,

    #[error("Service unavailable")]
    ServiceUnavailable,
}

impl AppError {
    /// Helper để tạo validation error với field-level details
    pub fn validation_fields(fields: HashMap<String, Vec<String>>) -> Self {
        AppError::ValidationFields(fields)
    }
}

// Error response body
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, Vec<String>>>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // ValidationFields cần xử lý riêng vì có fields
        if let AppError::ValidationFields(fields) = self {
            let body = Json(ErrorResponse {
                error: "Validation failed".to_string(),
                code: "VALIDATION_ERROR".to_string(),
                details: None,
                fields: Some(fields),
            });
            return (StatusCode::BAD_REQUEST, body).into_response();
        }

        let (status, error_message, code, details) = match self {
            // Auth errors
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized",
                "UNAUTHORIZED",
                Some(msg),
            ),
            AppError::Forbidden(msg) => {
                (StatusCode::FORBIDDEN, "Forbidden", "FORBIDDEN", Some(msg))
            }
            AppError::InvalidCredentials(msg) => (
                StatusCode::UNAUTHORIZED,
                "Invalid credentials",
                "INVALID_CREDENTIALS",
                Some(msg),
            ),

            // Validation errors
            AppError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                "Validation failed",
                "VALIDATION_ERROR",
                Some(msg),
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "Bad request",
                "BAD_REQUEST",
                Some(msg),
            ),

            // Resource errors
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "Resource not found",
                "NOT_FOUND",
                Some(msg),
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                "Resource conflict",
                "CONFLICT",
                Some(msg),
            ),

            // External service errors - log internally, ẩn details khỏi client
            AppError::EmailService(ref msg) => {
                tracing::error!("Email service error: {}", msg);
                (
                    StatusCode::BAD_GATEWAY,
                    "Email service unavailable",
                    "EMAIL_SERVICE_ERROR",
                    None,
                )
            }
            AppError::StorageService(ref msg) => {
                tracing::error!("Storage service error: {}", msg);
                (
                    StatusCode::BAD_GATEWAY,
                    "Storage service unavailable",
                    "STORAGE_SERVICE_ERROR",
                    None,
                )
            }
            AppError::Redis(ref msg) => {
                tracing::error!("Redis error: {}", msg);
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Cache service unavailable",
                    "CACHE_SERVICE_ERROR",
                    None,
                )
            }

            // Database errors - log internally, không leak gì ra ngoài
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    "DATABASE_ERROR",
                    None,
                )
            }

            // Config errors - log internally, không leak gì ra ngoài
            AppError::Config(ref e) => {
                tracing::error!("Configuration error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    "CONFIG_ERROR",
                    None,
                )
            }

            // Business logic
            AppError::BusinessLogic(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Business logic error",
                "BUSINESS_LOGIC_ERROR",
                Some(msg),
            ),

            // Generic errors
            AppError::ServiceUnavailable => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Service unavailable",
                "SERVICE_UNAVAILABLE",
                None,
            ),
            AppError::Internal => {
                tracing::error!("Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    "INTERNAL_ERROR",
                    None,
                )
            }

            // Đã xử lý ở trên, không bao giờ reach đến đây
            AppError::ValidationFields(_) => unreachable!(),
        };

        let body = Json(ErrorResponse {
            error: error_message.to_string(),
            code: code.to_string(),
            details,
            fields: None,
        });

        (status, body).into_response()
    }
}
