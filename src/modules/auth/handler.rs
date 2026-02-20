use axum::http::HeaderMap;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use validator::Validate;

use crate::app_state::AppState;
use crate::error::{AppError, ErrorResponse};
use crate::modules::auth::dto::{
    LoginRequest, RegisterRequest, ResendVerifyEmailRequest, VerifyEmailRequest,
};
use crate::modules::auth::repository::UserRepository;
use crate::modules::auth::service::AuthService;
use crate::shared::utils::flattern_error::flatten_errors;

//API: [POST] - api/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Nếu dữ liệu đầu vào sai (ví dụ thiếu email, password ngắn), trả về lỗi 400 ngay
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

    // 2. Dependency Injection (Thủ công)
    // Khởi tạo Repo và Service, truyền các phụ tùng từ AppState vào
    let user_repo = Arc::new(UserRepository::new(state.db.clone()));
    // state.email_service đã là Arc<dyn MailService> rồi, truyền thẳng vào luôn
    let auth_service = AuthService::new(
        user_repo,
        state.mail_service.clone(),
        state.token_service.clone(),
        state.config.clone(),
    );

    // 3. Gọi Business Logic
    // Dấu `?` ở đây sẽ làm 2 việc:
    // - Nếu thành công: Lấy `new_user` ra và chạy tiếp dòng dưới.
    // - Nếu thất bại (AuthError): Tự động chuyển thành `AppError` và return lỗi ngay lập tức.
    let new_user = auth_service.register(payload).await?;

    // 4. Thành công -> Trả về 201 Created cùng thông tin User
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

    let user_repo = Arc::new(UserRepository::new(state.db.clone()));

    let auth_service = AuthService::new(
        user_repo,
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

    let user_repo = Arc::new(UserRepository::new(state.db.clone()));

    let auth_service = AuthService::new(
        user_repo,
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

    // user-agent from header - (to know which browser people use)
    let user_agent = headers
        .get("User-Agent")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("Unkown")
        .to_string();

    let user_repo = Arc::new(UserRepository::new(state.db.clone()));
    let auth_service = AuthService::new(
        user_repo,
        state.mail_service.clone(),
        state.token_service.clone(),
        state.config.clone(),
    );

    let token_pair = auth_service.login(payload, user_agent).await?;

    let expires_in_minutes: u64 = state.config.jwt_expired_in.parse().unwrap_or(60);
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
