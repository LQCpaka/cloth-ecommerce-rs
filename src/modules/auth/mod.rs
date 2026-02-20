pub mod dto;
pub mod error;
pub mod guard;
pub mod handler;
pub mod repository;
pub mod service;

use axum::{Router, routing::post};
pub use repository::*;

use crate::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handler::register))
        .route("/verify", post(handler::verify))
        .route("/verify/resend", post(handler::resend_verification_email))
        .route("/login", post(handler::login))
}
