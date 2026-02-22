use axum::http::HeaderMap;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use std::time::Duration;
use validator::Validate;

use crate::app_state::AppState;
use crate::error::{AppError, ErrorResponse};
use crate::modules::auth::dto::{
    LoginRequest, RegisterRequest, ResendVerifyEmailRequest, VerifyEmailRequest,
};
use crate::modules::auth::repository::AuthRepository;
use crate::modules::auth::service::AuthService;
use crate::shared::services::redis_service::RedisService;
use crate::shared::utils::flattern_error::flatten_errors;

//API: [POST] - api/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate error
    if let Err(e) = payload.validate() {
        let clean_errors = flatten_errors(e);
        let response = ErrorResponse {
            error: "Validation Failed".to_string(),
            code: "VALIDATION_FAILED".to_string(),
            details: Some("Dữ liệu đầu vào không hợp lệ".to_string()),
            fields: Some(clean_errors),
        };

        return Ok((StatusCode::BAD_REQUEST, Json(response)).into_response());
    }

    let auth_repo = state.auth_repo;
    let auth_service = AuthService::new(
        auth_repo,
        state.mail_service.clone(),
        state.token_service.clone(),
        state.config.clone(),
    );

    let new_user = auth_service.register(payload).await?;

    Ok((StatusCode::CREATED, Json(new_user)).into_response())
}

// API: [POST] - api/auth/verify/
pub async fn verify(
    State(state): State<AppState>,
    Json(payload): Json<VerifyEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    //Validate Input - payload
    if let Err(e) = payload.validate() {
        let clean_errors = flatten_errors(e);
        let response = ErrorResponse {
            error: "Validation Failed".to_string(),
            code: "VALIDATION_FAILED".to_string(),
            details: Some("Dữ liệu xác thực không hợp lệ".to_string()),
            fields: Some(clean_errors),
        };
        return Ok((StatusCode::BAD_REQUEST, Json(response)).into_response());
    }

    let auth_repo = state.auth_repo;

    let auth_service = AuthService::new(
        auth_repo,
        state.mail_service.clone(),
        state.token_service.clone(),
        state.config.clone(),
    );

    let message = auth_service.verify_account(payload).await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status" : "success",
            "message": message
        })),
    )
        .into_response())
}

// API: [POST] - api/auth/verify/resend
pub async fn resend_verification_email(
    State(state): State<AppState>,
    Json(payload): Json<ResendVerifyEmailRequest>,
) -> Result<impl IntoResponse, AppError> {
    //Validate input
    if let Err(e) = payload.validate() {
        let clean_errors = flatten_errors(e);
        let response = ErrorResponse {
            error: "Validation Failed".to_string(),
            code: "VALIDATION_FAILED".to_string(),
            details: Some("Dữ liệu xác thực không hợp lệ".to_string()),
            fields: Some(clean_errors),
        };
        return Ok((StatusCode::BAD_REQUEST, Json(response)).into_response());
    }

    let redis_service = RedisService::new(state.redis_pool.clone());
    let rate_limit_key = format!("ratelimit:resend:{}", payload.email);
    redis_service
        .rate_limit(&rate_limit_key, 3, Duration::from_secs(300))
        .await?;

    let auth_repo = state.auth_repo;

    let auth_service = AuthService::new(
        auth_repo,
        state.mail_service.clone(),
        state.token_service.clone(),
        state.config.clone(),
    );

    let message = auth_service.resend_verification_email(payload).await?;
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": message
        })),
    )
        .into_response())
}

// API: [POST] - api/auth/login
pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    // validate
    if let Err(e) = payload.validate() {
        let clean_errors = flatten_errors(e);
        return Err(AppError::validation_fields(clean_errors));
    }

    let redis_service = RedisService::new(state.redis_pool.clone());
    let rate_limit_key = format!("ratelimit:login:{}", payload.email);
    redis_service
        .rate_limit(&rate_limit_key, 3, Duration::from_secs(300))
        .await?;
    // user-agent from header - (to know which browser people use)
    let user_agent = headers
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unkown")
        .to_string();

    let auth_repo = Arc::new(AuthRepository::new(state.db.clone()));

    let auth_service = AuthService::new(
        auth_repo,
        state.mail_service.clone(),
        state.token_service.clone(),
        state.config.clone(),
    );

    let token_pair = auth_service.login(payload, user_agent).await?;

    let expires_in_minutes: u64 = state.config.jwt_expired_in as u64;
    let expires_in_seconds = expires_in_minutes * 60;

    Ok(Json(serde_json::json!({
        "status": "success",
        "data": {
            "access_token": token_pair.access_token,
            "refresh_token": token_pair.refresh_token,
            "token_type": "Bearer",
            "expires_in": expires_in_seconds
        }
    })))
}
