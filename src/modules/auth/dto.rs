use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, message = "Tên không được để trống"))]
    pub name: String,

    #[validate(email(message = "Email không hợp lệ"))]
    pub email: String,

    #[validate(
        length(min = 6, max = 20, message = "Mật khẩu phải từ 6-20 ký tự"),
        custom(function = "validate_password")
    )]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct Login {
    #[validate(email(message = "Email không hợp lệ"))]
    pub email: String,

    #[validate(
        length(min = 6, max = 20, message = "Mật khẩu phải từ 6-20 ký tự"),
        custom(function = "validate_password")
    )]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if !has_letter || !has_digit {
        let mut error = ValidationError::new("password_complexity");
        error.message = Some(Cow::from("Mật khẩu phải chứa cả chữ cái và số"));
        return Err(error);
    }

    Ok(())
}
