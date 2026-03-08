use axum::{
    Router,
    routing::{get, post},
};

use crate::{app_state::AppState, modules::category::handler};

pub fn category_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::get_category_tree))
        .route("/", post(handler::create_category))
}
