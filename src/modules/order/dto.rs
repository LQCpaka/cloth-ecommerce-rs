use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CheckoutRequest {
    pub address_id: Uuid,
    pub note: Option<String>,
}
