use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{
    app_state::AppState,
    error::AppError,
    modules::{admin::dto::DeleteAccountRequest, auth::guard::AuthUser, user::model::UserRole},
};

//API: [DELETE] - api/v1/admin/delete-user
pub async fn delete_user(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<DeleteAccountRequest>,
) -> Result<impl IntoResponse, AppError> {
    user.require_roles(&[UserRole::Admin, UserRole::Moderator])?;

    state.admin_repo.delete_user(&payload.email).await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "message": "Xóa user thành công!"
        })),
    ))
}
