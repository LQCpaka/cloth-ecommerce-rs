use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "axum_ecommerce=debug,tower_http=debug,axum=debug,sqlx=warn".into());

    let is_production = std::env::var("APP_ENV")
        .map(|v| v == "production")
        .unwrap_or(false);

    let registry = tracing_subscriber::registry().with(env_filter);

    if is_production {
        registry
            .with(
                tracing_subscriber::fmt::layer()
                    .json()
                    .with_target(true)
                    .with_current_span(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
    } else {
        registry
            .with(
                tracing_subscriber::fmt::layer()
                    .pretty() // dễ đọc ở terminal
                    .with_target(true)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
    }
}
