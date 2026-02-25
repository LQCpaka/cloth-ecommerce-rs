use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    app_state::AppState,
    error::AppError,
    modules::{
        auth::guard::AuthUser,
        category::{dto::CreateCategoryRequest, service::CategoryService},
        user::model::UserRole,
    },
};

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

// API: POST /api/v1/categories
pub async fn create_category(
    State(state): State<AppState>,
    user: AuthUser, // Require login Bearer
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Phân quyền sơ bộ: Chỉ Admin hoặc Seller mới được phép qua
    if user.role != UserRole::Admin && user.role != UserRole::Seller {
        return Err(AppError::Unauthorized(
            "Bạn không có quyền thực hiện chức năng này!".to_string(),
        ));
    }

    // 2. Kiểm tra dữ liệu hợp lệ (Validation)
    if let Err(e) = payload.validate() {
        // Vợ dùng lại hàm xử lý lỗi validation giống bên Auth nha
        // Chồng viết tạm trả về BadRequest cho gọn
        return Err(AppError::BadRequest(
            "Dữ liệu không hợp lệ. Vui lòng kiểm tra lại tên danh mục!".to_string(),
        ));
    }

    // 3. Khởi tạo Service và lưu vào DB
    let category_service = CategoryService::new(state.category_repo.clone());
    let new_category = category_service.create_category(payload).await?;

    // 4. Trả về kết quả
    Ok((
        StatusCode::CREATED, // HTTP 201 Created
        Json(serde_json::json!({
            "status": "success",
            "message": "Tạo danh mục thành công!",
            "data": new_category
        })),
    )
        .into_response())
}
