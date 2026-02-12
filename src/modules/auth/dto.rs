use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Deserialize, Serialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, message = "Name can't be empty"))]
    pub name: String,

    #[validate(email(message = "Invalid Email"))]
    pub email: String,

    #[validate(length(min = 6, max = 20), custom(function = "validate_password"))]
    pub password: String,
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    //Just incase length doesnt work or overrdided by something else, idk
    if password.len() < 6 || password.len() > 20 {
        return Err(ValidationError::new("length"));
    }

    let has_letter = password.chars().any(|c| c.is_ascii_alphabetic());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if !has_letter || has_digit {
        return Err(ValidationError::new("password_format"));
    }

    Ok(())
}
