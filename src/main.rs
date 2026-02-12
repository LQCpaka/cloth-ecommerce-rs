mod app_state;
mod config;
mod modules;
mod shared;

use crate::app_state::AppState;
use axum::Router;
use config::Config;
use log::LevelFilter;
use sqlx::ConnectOptions;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
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
        .connect_with(db_options.log_statements(LevelFilter::Off)) // <-- Gọi liền ở đây luôn
        .await
        .expect("Failed to connect to Database");

    tracing::info!("Successfuly Connected to Database!");

    // Create AppState
    let state = AppState { db: pool };

    //Define Router (Sample, will change soon)
    let app = Router::new()
        // Gắn module Auth vào đường dẫn /api/auth
        // -> API sẽ là: POST http://localhost:3000/api/auth/register
        .nest("/api/auth", modules::auth::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = format!("{}:{}", config.server_host, config.server_port);
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
