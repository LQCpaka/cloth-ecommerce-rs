use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Security Error: {0}")]
    SecurityError(String),

    #[error("Database Error: {0}")]
    DatabaseError(String),
}

//Convert sqlx::Error -> AuthErr
impl From<sqlx::Error> for AuthError {
    fn from(err: sqlx::Error) -> Self {
        AuthError::DatabaseError(err.to_string())
    }
}
