use dotenvy::dotenv;
use std::env;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing enviroment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid port number: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
    #[error("Environment error: {0}")]
    EnvError(#[from] dotenvy::Error),
}
impl Config {
    pub fn init() -> Result<Config, ConfigError> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?;
        let server_host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()?;

        Ok(Config {
            database_url,
            server_host,
            server_port,
        })
    }
}
