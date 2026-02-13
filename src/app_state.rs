use crate::shared::ports::mail::EmailService;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub email_service: Arc<dyn EmailService>,
}
