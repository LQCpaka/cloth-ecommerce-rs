use thiserror::Error;

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Handling failed")]
    HashError,
    #[error("Invalid credential")]
    InvalidCredential,
}
