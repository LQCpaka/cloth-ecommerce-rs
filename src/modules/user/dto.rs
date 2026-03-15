use crate::modules::user::model::UserRole;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
    pub description: Option<String>,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateAddressRequest {
    #[validate(length(min = 2, message = "Tên người nhận quá ngắn"))]
    pub recipient_name: String,

    #[validate(length(min = 10, message = "Số điện thoại không hợp lệ"))]
    pub recipient_phone: String,

    pub address_line: String, // Số nhà, tên đường
    pub ward: Option<String>, // Phường/Xã
    pub district: String,     // Quận/Huyện
    pub city: String,         // Tỉnh/Thành Phố

    pub is_default: Option<bool>, // Set làm địa chỉ mặc định luôn không?
}
