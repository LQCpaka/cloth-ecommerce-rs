use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use validator::Validate;

use crate::{
    app_state::AppState,
    error::AppError,
    modules::{
        auth::guard::AuthUser,
        product::{dto::CreateProductRequest, service::ProductService},
        user::model::UserRole,
    },
};

//API: POST api/v1/products
pub async fn create_product(
    State(state): State<AppState>,
    user: AuthUser,
    Json(payload): Json<CreateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Auth Guard
    user.require_roles(&[UserRole::Admin, UserRole::Seller, UserRole::Moderator])?;

    //Validate input
    if let Err(_e) = payload.validate() {
        return Err(AppError::BadRequest(
            "Dữ liệu không liệu không hợp lệ. Vui lòng kiểm tra lại!".to_string(),
        ));
    }

    // handle create product logics
    let product_service = ProductService::new(state.product_repo.clone());
    let new_product = product_service.create_product(payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "status": "success",
            "message": "Tạo sản phẩm nháp thành công!",
            "data": new_product
        })),
    )
        .into_response())
}
