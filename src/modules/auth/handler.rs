use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use std::sync::Arc;
use validator::Validate;

use crate::app_state::AppState;
use crate::error::AppError;
use crate::modules::auth::dto::RegisterRequest;
use crate::modules::auth::repository::UserRepository;
use crate::modules::auth::service::AuthService;

//API: [POST] - api/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    // 1. Validate Input (Giữ nguyên)
    // Nếu dữ liệu đầu vào sai (ví dụ thiếu email, password ngắn), trả về lỗi 400 ngay
    if let Err(e) = payload.validate() {
        // Trả về Ok(...) vì đây là phản hồi HTTP hợp lệ (dù là mã lỗi 400)
        return Ok((StatusCode::BAD_REQUEST, Json(e)).into_response());
    }

    // 2. Dependency Injection (Thủ công)
    // Khởi tạo Repo và Service, truyền các phụ tùng từ AppState vào
    let user_repo = Arc::new(UserRepository::new(state.db.clone()));

    // state.email_service đã là Arc<dyn MailService> rồi, truyền thẳng vào luôn
    let auth_service = AuthService::new(user_repo, state.mail_service.clone());

    // 3. Gọi Business Logic
    // Dấu `?` ở đây sẽ làm 2 việc:
    // - Nếu thành công: Lấy `new_user` ra và chạy tiếp dòng dưới.
    // - Nếu thất bại (AuthError): Tự động chuyển thành `AppError` và return lỗi ngay lập tức.
    let new_user = auth_service.register(payload).await?;

    // 4. Thành công -> Trả về 201 Created cùng thông tin User
    Ok((StatusCode::CREATED, Json(new_user)).into_response())
}
