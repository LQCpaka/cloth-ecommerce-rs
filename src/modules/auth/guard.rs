use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, Validation, decode};
use uuid::Uuid;

use crate::{
    app_state::AppState, error::AppError, modules::user::model::UserRole,
    shared::services::jwt::Claims,
};

pub struct AuthUser {
    pub id: Uuid,
    pub role: UserRole,
    pub email: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Find header "Authorization"
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                AppError::Unauthorized("Thiếu token xác thực. Vui lòng đăng nhập!".to_string())
            })?;
        //Validate if there is "Bearer"
        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::Unauthorized(
                "Sai định dạng token. Phải bắt đầu bằng Bearer".to_string(),
            ));
        }
        // trim "Bearer" - Take token part
        let token = auth_header.trim_start_matches("Bearer ");
        let secret = &state.config.jwt_secret;
        // handle token - decode
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Token không hợp lệ hoặc đã hết hạn".to_string()))?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AppError::Unauthorized("ID người dùng không hợp lệ".to_string()))?;

        //Allow user go through and give data to handler
        Ok(AuthUser {
            id: user_id,
            role: token_data.claims.role,
            email: token_data.claims.email,
        })
    }
}

impl AuthUser {
    pub fn require_roles(&self, allowed_roles: &[UserRole]) -> Result<(), AppError> {
        if !allowed_roles.contains(&self.role) {
            tracing::warn!(
                "Báo động: Role '{:?}' cố gắng truy cập trái phép!",
                self.role
            );
            return Err(AppError::Unauthorized(
                "Bạn không đủ quyền để thực hiện chức năng này!".to_string(),
            ));
        }
        Ok(())
    }
}
