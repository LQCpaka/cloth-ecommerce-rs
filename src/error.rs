use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing enviroment variable: {0}")]
    MissingEnvVar(String),
    #[error("Invalid port number: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
    #[error("Environment error: {0}")]
    EnvError(#[from] dotenvy::Error),
}
