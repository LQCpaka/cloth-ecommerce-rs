mod app_state;
mod config;
mod error;
mod infrastructure;
mod modules;
mod shared;

use crate::app_state::AppState;

use axum::Router;
use config::Config;
use log::LevelFilter;
use sqlx::ConnectOptions;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    tracing::info!("Starting Server... ");

    let db_options: sqlx::postgres::PgConnectOptions = config.database_url.parse()?;

    // 2. Kết nối và gọi log_statements ngay bên trong đó
    let pool = PgPoolOptions::new()
        .max_connections(10)
        //Turn off [DEBUG] querry cause eyes config
        .connect_with(db_options.log_statements(LevelFilter::Off))
        .await
        .expect("Failed to connect to Database");

    tracing::info!("Successfuly Connected to Database!");

    //Email service
    tracing::info!("Email service initialized");
    // Create AppState
    let state = AppState::new(config, pool);
    let addr = format!("{}:{}", state.config.server_host, state.config.server_port);
    let app = Router::new()
        .nest("/api/auth", modules::auth::router())
        .with_state(state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO)) // INFO thay vì DEBUG
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        );

    // Start server
    let addr = format!("{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server is listening at: {}", addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to Ctrl+C signal handler");
    tracing::info!("Shutting down server....");
}
