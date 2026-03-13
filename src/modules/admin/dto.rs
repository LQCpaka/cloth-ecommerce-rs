use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteAccountRequest {
    pub email: String,
}
