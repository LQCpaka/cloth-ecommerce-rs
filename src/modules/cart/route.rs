use axum::{
    Router,
    routing::{post, put},
};

use crate::{app_state::AppState, modules::cart::handler};

pub fn cart_router() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::add_to_cart).get(handler::get_cart))
        .route(
            "/{variant_id}",
            put(handler::update_cart_item).delete(handler::remove_from_cart),
        )
}
