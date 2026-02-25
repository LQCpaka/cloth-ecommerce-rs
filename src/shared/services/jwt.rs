use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use rand::RngExt;
use serde::{Deserialize, Serialize};

use crate::modules::user::model::UserRole;

// Access Token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub role: UserRole,
    pub email: String,
}

#[derive(Clone)]
pub struct TokenService {
    secret: String,
    expires_in_minutes: i64,
}

impl TokenService {
    pub fn new(secret: String, expires_in_minutes: i64) -> Self {
        Self {
            secret,
            expires_in_minutes,
        }
    }

    pub fn generate_access_token(
        &self,
        user_id: uuid::Uuid,
        role: UserRole,
        email: String,
    ) -> Result<String, String> {
        let now = Utc::now();
        let expiry = now + Duration::minutes(self.expires_in_minutes);

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp() as usize,
            exp: expiry.timestamp() as usize,
            role,
            email,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| e.to_string())
    }

    //refresh token
    pub fn generate_refresh_token(&self) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::rng();

        (0..64)
            .map(|_| {
                let idx = rng.random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}
