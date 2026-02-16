use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    config::Config, infrastructure::mail::ResendMailService, shared::ports::mail::MailService,
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub mail_service: Arc<dyn MailService>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(config: Config, db: PgPool) -> Self {
        let config = Arc::new(config);

        let email = format!("Shop Cua Vo <{}>", config.from_email);
        let mail_service: Arc<dyn MailService> =
            Arc::new(ResendMailService::new(config.resend_api_key.clone(), email));

        Self {
            db,
            mail_service,
            config,
        }
    }
}
