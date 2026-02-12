use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use validator::Validate;

use crate::app_state::AppState;
use crate::modules::auth::dto::RegisterRequest;
use crate::modules::auth::error::AuthError;
use crate::modules::auth::repository::UserRepository;
use crate::modules::auth::service::AuthService;

//API: [POST] - api/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    //Dependency Injection
    let user_repo = Arc::new(UserRepository::new(state.db.clone()));
    let auth_service = AuthService::new(user_repo);

    if let Err(e) = payload.validate() {
        return (StatusCode::BAD_REQUEST, Json(e)).into_response();
    }

    match auth_service.register(payload).await {
        Ok(new_user) => {
            // 201 - Successful, return json user data(except the pass)
            (StatusCode::CREATED, Json(new_user)).into_response()
        }
        Err(e) => {
            let status = match e {
                AuthError::EmailAlreadyExists => StatusCode::CONFLICT,
                AuthError::SecurityError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            };

            (status, Json(serde_json::json!({"error": e.to_string()}))).into_response()
        }
    }
}
