use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    config::Config,
    infrastructure::mail::ResendMailService,
    shared::{ports::mail::MailService, services::jwt::TokenService},
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub mail_service: Arc<dyn MailService>,
    pub token_service: Arc<TokenService>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(config: Config, db: PgPool) -> Self {
        let config = Arc::new(config);

        let email = format!("Shop Cua Vo <{}>", config.from_email);
        let mail_service: Arc<dyn MailService> =
            Arc::new(ResendMailService::new(config.resend_api_key.clone(), email));
        let token_service: Arc<TokenService> = Arc::new(TokenService::new(
            config.jwt_secret.clone(),
            config.jwt_expired_in.clone(),
        ));
        Self {
            db,
            mail_service,
            token_service,
            config,
        }
    }
}
