use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{app_state::AppState, error::AppError, modules::product::service::CategoryService};

// API: GET /api/v1/categories
pub async fn get_category_tree(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let category_service = CategoryService::new(state.category_repo.clone());

    let tree = category_service.get_category_tree().await?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "success",
            "data": tree
        })),
    )
        .into_response())
}
