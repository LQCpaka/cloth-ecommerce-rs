use crate::{app_state::AppState, modules};
use axum::{
    Router,
    http::{HeaderName, HeaderValue, Request},
};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use uuid::Uuid;

static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

#[derive(Clone)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string();
        HeaderValue::from_str(&id).ok().map(RequestId::new)
    }
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1", api_v1())
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(SetRequestIdLayer::new(
                    X_REQUEST_ID.clone(),
                    MakeRequestUuid,
                ))
                .layer(PropagateRequestIdLayer::new(X_REQUEST_ID.clone()))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let request_id = request
                                .headers()
                                .get("x-request-id")
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("unknown");
                            tracing::info_span!(
                                "http_request",
                                request_id = %request_id,
                                method = %request.method(),
                                uri = %request.uri(),
                            )
                        })
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                ),
        )
}

fn api_v1() -> Router<AppState> {
    Router::new()
        .nest("/auth", modules::auth::router())
        .nest("/users", modules::user::router())
        .nest("/categories", modules::product::category_router())
}
