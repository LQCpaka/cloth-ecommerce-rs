use axum::{Router, routing::post};

use crate::{app_state::AppState, modules::product::handler};

pub fn product_router() -> Router<AppState> {
    Router::new().route("/", post(handler::create_product))
}
