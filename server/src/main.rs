//! Telemetry Kit Ingestion Server

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod clacks;
mod config;
mod handlers;
mod models;

use config::Config;

/// Application state
pub struct AppState {
    db: sqlx::PgPool,
    config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::load()?;

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| config.server.log_level.clone().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!(
        "Starting telemetry-kit-server v{}",
        env!("CARGO_PKG_VERSION")
    );

    // Connect to database
    tracing::info!("Connecting to database...");
    let db = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;

    // Run migrations
    tracing::info!("Running database migrations...");
    run_migrations(&db).await?;

    // Create application state
    let state = Arc::new(AppState {
        db,
        config: config.clone(),
    });

    // Build router with state
    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health))
        // Ingestion endpoint with middleware
        .route(
            "/v1/ingest/:org_id/:app_id",
            post(handlers::ingest)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    auth::verify_hmac,
                )),
        )
        .with_state(state)
        // Add CORS
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        // Add GNU Terry Pratchett header (http://www.gnuterrypratchett.com/)
        .layer(middleware::from_fn(clacks::add_clacks_header))
        // Add tracing
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = config.server.address();
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Run database migrations
async fn run_migrations(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    // Read migration file
    let migration = include_str!("../migrations/001_init.sql");

    // Split by semicolons and execute each statement
    for statement in migration.split(';') {
        let statement = statement.trim();
        if !statement.is_empty() && !statement.starts_with("--") {
            sqlx::query(statement).execute(pool).await.ok();
        }
    }

    Ok(())
}
