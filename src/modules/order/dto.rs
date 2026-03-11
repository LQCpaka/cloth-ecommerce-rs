use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CheckoutRequest {
    #[validate(length(min = 2, message = "Tên người nhận phải từ 2 ký tự trở lên"))]
    pub receiver_name: String,

    #[validate(length(min = 10, message = "Số điện thoại không hợp lệ"))]
    pub receiver_phone: String,

    #[validate(length(
        min = 10,
        message = "Địa chỉ giao hàng quá ngắn, shipper tìm không ra đâu!"
    ))]
    pub shipping_address: String,

    // Khách có dặn dò gì thêm không? (Ví dụ: "Giao giờ hành chính nha shop")
    pub note: Option<String>,
}
