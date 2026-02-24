use axum::{Router, routing::get};

use crate::app_state::AppState;

pub mod dto;
pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

pub fn category_router() -> Router<AppState> {
    Router::new().route("/", get(handler::get_category_tree))
}
