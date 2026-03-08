use axum::{
    Router,
    routing::{get, post},
};

use crate::{app_state::AppState, modules::product::handler};

pub fn product_router() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_product))
        .route("/", get(handler::get_products))
        .route("/:id", get(handler::get_product_detail))
        .route("/{product_id}/variants", post(handler::create_variant))
        .route("/{product_id}/images", post(handler::upload_product_image))
}
