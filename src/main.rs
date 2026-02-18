mod app_state;
mod config;
mod error;
mod infrastructure;
mod modules;
mod shared;

use crate::app_state::AppState;
use axum::{
    Router,
    http::{HeaderName, HeaderValue, Request},
};
use config::Config;
use sqlx::ConnectOptions;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

// Header name cho request ID
static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

// Struct để generate UUID cho mỗi request
#[derive(Clone)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _: &Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string();
        HeaderValue::from_str(&id).ok().map(RequestId::new)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load config
    let config = Config::init()?;

    // Start logger
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_ecommerce=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Server...");

    // Kết nối DB, tắt log query
    let db_options: sqlx::postgres::PgConnectOptions = config.database_url.parse()?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(db_options.log_statements(log::LevelFilter::Off))
        .await
        .expect("Failed to connect to Database");

    tracing::info!("Successfully connected to Database!");
    tracing::info!("Email service initialized");

    let state = AppState::new(config, pool);
    let addr = format!("{}:{}", state.config.server_host, state.config.server_port);

    let app = Router::new()
        .nest("/api/auth", modules::auth::router())
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                // 1. Gắn request ID vào mỗi request
                .layer(SetRequestIdLayer::new(
                    X_REQUEST_ID.clone(),
                    MakeRequestUuid,
                ))
                // 2. Propagate request ID vào response header
                // → client nhận được x-request-id để debug
                .layer(PropagateRequestIdLayer::new(X_REQUEST_ID.clone()))
                // 3. Trace layer với request ID trong span
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
        );

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening at: {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    tracing::info!("Shutting down server...");
}
