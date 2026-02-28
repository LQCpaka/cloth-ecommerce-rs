use crate::{app_state::AppState, modules::user::handler};
use axum::{Router, routing::get};

pub fn user_router() -> Router<AppState> {
    Router::new().route("/me", get(handler::get_my_profile))
}
