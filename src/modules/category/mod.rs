pub mod dto;
pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

use axum::{
    Router,
    routing::{get, post},
};

use crate::app_state::AppState;

pub fn category_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::get_category_tree))
        .route("/", post(handler::create_category))
}
