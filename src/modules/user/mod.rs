use axum::{Router, routing::get};

use crate::app_state::AppState;

pub mod dto;
pub mod handler;
pub mod model;
pub mod repository;

pub fn router() -> Router<AppState> {
    Router::new().route("/me", get(handler::get_my_profile))
}
