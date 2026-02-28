use crate::{app_state::AppState, modules::auth::handler};
use axum::{Router, routing::post};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handler::register))
        .route("/verify", post(handler::verify))
        .route("/verify/resend", post(handler::resend_verification_email))
        .route("/login", post(handler::login))
}
