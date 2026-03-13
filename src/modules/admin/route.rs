use axum::{Router, routing::delete};

use crate::{app_state::AppState, modules::admin::handler};

pub fn admin_router() -> Router<AppState> {
    Router::new().route("/delete-user", delete(handler::delete_user))
}
