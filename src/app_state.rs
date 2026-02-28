use deadpool_redis::Pool;
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    config::Config,
    infrastructure::{mail::ResendMailService, redis::client::RedisInfra},
    modules::{
        auth::AuthRepository, category::repository::CategoryRepository,
        product::repository::ProductRepository, user::repository::UserRepository,
    },
    shared::{ports::mail::MailService, services::jwt::TokenService},
};

#[derive(Clone)]
pub struct AppState {
    // ======================| ENV |=========================
    pub config: Arc<Config>,
    // ======================| POOL |========================
    pub db: PgPool,
    pub redis_pool: Pool,
    // ==================| SERVICE STATE |===================
    pub mail_service: Arc<dyn MailService>,
    pub token_service: Arc<TokenService>,
    // ==================| REPO STATE |======================
    pub auth_repo: Arc<AuthRepository>,
    pub user_repo: Arc<UserRepository>,
    pub category_repo: Arc<CategoryRepository>,
    pub product_repo: Arc<ProductRepository>,
}

impl AppState {
    pub fn new(config: Config, db: PgPool) -> Self {
        let config = Arc::new(config);

        let email = format!("Shop Cua Vo <{}>", config.from_email);
        let mail_service: Arc<dyn MailService> =
            Arc::new(ResendMailService::new(config.resend_api_key.clone(), email));
        let redis_pool = RedisInfra::init_pool(&config.redis_url);
        let token_service: Arc<TokenService> = Arc::new(TokenService::new(
            config.jwt_secret.clone(),
            config.jwt_expired_in.clone(),
        ));
        //=======================================================
        // =====================| REPO STATE |===================
        //=======================================================
        let auth_repo: Arc<AuthRepository> = Arc::new(AuthRepository::new(db.clone()));
        let user_repo: Arc<UserRepository> = Arc::new(UserRepository::new(db.clone()));
        let category_repo: Arc<CategoryRepository> = Arc::new(CategoryRepository::new(db.clone()));
        let product_repo: Arc<ProductRepository> = Arc::new(ProductRepository::new(db.clone()));
        Self {
            db,
            redis_pool,
            mail_service,
            token_service,
            config,
            auth_repo,
            user_repo,
            category_repo,
            product_repo,
        }
    }
}
