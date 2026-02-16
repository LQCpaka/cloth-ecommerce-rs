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
    #[error("Missing enviroment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid port number: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
    #[error("Environment error: {0}")]
    EnvError(#[from] dotenvy::Error),
    #[error("Invalid email format")]
    InvalidEmailFormat(String),
}

// Application-wide error type
#[derive(Error, Debug)]
pub enum AppError {
    //Configuration errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    //Database errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    //Auth errors
    #[error("Authentication failed: {0}")]
    Unauthorized(String),

    #[error("Access Forbidden: {0}")]
    Forbidden(String),

    #[error("Invalid credentials: {0}")]
    InvalidCredentials(String),

    // Validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    // Resouce errors
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

//Error response body
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, Vec<String>>>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, details) = match self {
            // Auth errors
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "Unauthorized", Some(msg)),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "Forbidden", Some(msg)),
            AppError::InvalidCredentials(msg) => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials", Some(msg))
            }

            // Validation errors
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "Validation failed", Some(msg)),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "Bad request", Some(msg)),

            // Resource errors
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "Resource not found", Some(msg)),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "Resource conflict", Some(msg)),

            // External service errors
            AppError::EmailService(msg) => {
                (StatusCode::BAD_GATEWAY, "Email service error", Some(msg))
            }
            AppError::StorageService(msg) => {
                (StatusCode::BAD_GATEWAY, "Storage service error", Some(msg))
            }
            AppError::Redis(msg) => (StatusCode::BAD_GATEWAY, "Cache service error", Some(msg)),

            // Database errors
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error", None)
            }

            // Config errors
            AppError::Config(ref e) => {
                tracing::error!("Configuration error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Configuration error",
                    None,
                )
            }

            // Business logic
            AppError::BusinessLogic(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Business logic error",
                Some(msg),
            ),

            // Generic errors
            AppError::ServiceUnavailable => {
                (StatusCode::SERVICE_UNAVAILABLE, "Service unavailable", None)
            }
            AppError::Internal => {
                tracing::error!("Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    None,
                )
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
            details,
            fields: None,
        });

        (status, body).into_response()
    }
}
