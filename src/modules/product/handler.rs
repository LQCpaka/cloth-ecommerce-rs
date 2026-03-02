use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::AppState,
    error::AppError,
    modules::{
        auth::guard::AuthUser,
        product::{
            dto::{CreateProductRequest, CreateVariantRequest},
            service::ProductService,
        },
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

//API: POST api/v1/products/variants
pub async fn create_variant(
    State(state): State<AppState>,
    user: AuthUser,
    Path(product_id): Path<Uuid>,
    Json(payload): Json<CreateVariantRequest>,
) -> Result<impl IntoResponse, AppError> {
    user.require_roles(&[UserRole::Admin, UserRole::Moderator, UserRole::Seller])?;

    // input validate
    if let Err(_e) = payload.validate() {
        return Err(AppError::BadRequest(
            "Dữ liệu SKU không hợp lệ!".to_string(),
        ));
    }

    let product_service = ProductService::new(state.product_repo.clone());
    let new_variant = product_service.create_variant(product_id, payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "status": "success",
            "message": "Nhập kho phân loại thành công!",
            "data":new_variant
        })),
    )
        .into_response())
}

// API: POST /api/v1/products/:product_id/images
pub async fn upload_product_image(
    State(state): State<AppState>,
    user: AuthUser,
    Path(product_id): Path<Uuid>,
    multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    user.require_roles(&[UserRole::Admin, UserRole::Moderator, UserRole::Seller])?;

    // Calling UploadService to send file to Cloudflare R2
    let image_url = state.upload_service.upload_image(multipart).await?;

    // Save the link into db
    state
        .product_repo
        .add_product_image(product_id, &image_url)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({
            "status": "success",
            "message": "Upload ảnh lên mây thành công!",
            "data": {
                "product_id": product_id,
                "image_url": image_url
            }
        })),
    )
        .into_response())
}
