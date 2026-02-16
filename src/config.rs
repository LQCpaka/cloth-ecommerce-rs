use crate::error::ConfigError;

use dotenvy::dotenv;
use email_address::EmailAddress;
use std::{env, str::FromStr};

#[derive(Debug, Clone)]
pub struct Config {
    //db config env
    pub database_url: String,
    //domain - host config
    pub server_host: String,
    pub server_port: u16,
    pub domain_name: String,
    // Resend env
    pub resend_api_key: String,
    pub from_email: String,
}

impl Config {
    fn get_env(key: &str) -> Result<String, ConfigError> {
        // Take value if not -> return empty string
        let val = env::var(key).unwrap_or_default();
        if val.trim().is_empty() {
            Err(ConfigError::MissingEnvVar(key.to_string()))
        } else {
            Ok(val)
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

        // DB struct - Config Env
        let database_url = Self::get_env("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;
        let server_host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()?;
        let domain_name = Self::get_env("DOMAIN_NAME")?;
        // Email struct - Config Env
        let resend_api_key = Self::get_env("RESEND_API_KEY")?;

        let from_email = Self::get_email_env("FROM_EMAIL")?;

        Ok(Config {
            database_url,
            server_host,
            server_port,
            domain_name,
            resend_api_key,
            from_email,
        })
    }
}
