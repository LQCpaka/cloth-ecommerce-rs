use crate::shared::services::error::*;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub struct PasswordService;

impl PasswordService {
    pub fn hash_password(password: &str) -> Result<String, SecurityError> {
        let salt = SaltString::generate(&mut OsRng); // Get rng string
        let argon2 = Argon2::default(); //Default config

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| SecurityError::HashError)?
            .to_string();

        Ok(password_hash)
    }

    pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, SecurityError> {
        let parsed_hash = PasswordHash::new(password_hash).map_err(|_| SecurityError::HashError)?;

        let argon2 = Argon2::default();

        // Compare
        let is_valid = argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(is_valid)
    }
}
