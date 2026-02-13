use dotenvy::dotenv;
use std::env;

use crate::error::ConfigError;

#[derive(Debug, Clone)]
pub struct Config {
    //db config env
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,

    //email config env
    pub smtp_user: String,
    pub smtp_host: String,
    pub smtp_pass: String,
}

impl Config {
    pub fn init() -> Result<Config, ConfigError> {
        dotenv().ok();

        // DB struct - Config Env
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;
        let server_host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()?;

        // Email struct - Config Env
        let smtp_user = env::var("SMTP_USER")
            .map_err(|_| ConfigError::MissingEnvVar("SMTP_USER".to_string()))?;

        let smtp_host = env::var("SMTP_HOST")
            .map_err(|_| ConfigError::MissingEnvVar("SMTP_HOST".to_string()))?;

        let smtp_pass = env::var("SMTP_PASS")
            .map_err(|_| ConfigError::MissingEnvVar("SMTP_PASS".to_string()))?;

        Ok(Config {
            database_url,
            server_host,
            server_port,

            //email
            smtp_host,
            smtp_pass,
            smtp_user,
        })
    }
}
