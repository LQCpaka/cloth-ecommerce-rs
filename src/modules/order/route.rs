use axum::{
    Router,
    routing::{get, post},
};

use crate::{app_state::AppState, modules::order::handler};

pub fn order_router() -> Router<AppState> {
    Router::new().route("/checkout", post(handler::checkout))
}
