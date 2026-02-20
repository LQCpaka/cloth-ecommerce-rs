use crate::error::ConfigError;

use dotenvy::dotenv;
use email_address::EmailAddress;
use std::{env, str::FromStr};

#[derive(Debug, Clone)]
pub struct Config {
    //db config env
    pub database_url: String,
    //domain - host env
    pub server_host: String,
    pub server_port: u16,
    pub domain_name: String,
    // Resend env
    pub resend_api_key: String,
    pub from_email: String,
    //jwt env
    pub jwt_secret: String,
    pub jwt_expired_in: i64,
    pub refresh_token_expired_in: i32,
}

impl Config {
    fn get_env(key: &str) -> Result<String, ConfigError> {
        // Take value if not -> return empty string
        match env::var(key) {
            Ok(val) if !val.trim().is_empty() => Ok(val),
            Ok(_) => Err(ConfigError::MissingEnvVar(key.to_string())),
            Err(_) => Err(ConfigError::MissingEnvVar(key.to_string())),
        }
    }

    fn get_email_env(key: &str) -> Result<String, ConfigError> {
        let email = Self::get_env(key)?;

        EmailAddress::from_str(&email)
            .map_err(|_| ConfigError::InvalidEmailFormat(email.clone()))?;

        Ok(email)
    }

    pub fn init() -> Result<Config, ConfigError> {
        dotenv().ok();

        Ok(Config {
            database_url: Self::get_env("DATABASE_URL")?,
            server_host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: Self::get_env("PORT")?
                .parse::<u16>()
                .map_err(|e| ConfigError::InvalidNumber("PORT".into(), e.to_string()))?,
            domain_name: Self::get_env("DOMAIN_NAME")?,
            resend_api_key: Self::get_env("RESEND_API_KEY")?,
            from_email: Self::get_email_env("FROM_EMAIL")?,
            jwt_secret: Self::get_env("JWT_SECRET")?,
            jwt_expired_in: Self::get_env("JWT_EXPIRED_IN")?
                .parse::<i64>()
                .map_err(|e| ConfigError::InvalidNumber("JWT_EXPIRED_IN".into(), e.to_string()))?,
            refresh_token_expired_in: Self::get_env("REFRESH_TOKEN_EXPIRED_IN")?
                .parse::<i32>()
                .map_err(|e| {
                    ConfigError::InvalidNumber("REFRESH_TOKEN_EXPIRED_IN".into(), e.to_string())
                })?,
        })
    }
}
