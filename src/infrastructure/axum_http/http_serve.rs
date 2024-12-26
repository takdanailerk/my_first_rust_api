use std::{net::SocketAddr, sync::Arc, time::Duration};
use anyhow::Result;
use axum::{http::Method, routing::get, Router};
use tokio::net::TcpListener;
use tower_http::{cors::{Any, CorsLayer}, limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;

use crate::{config::config_model::DotEnvyConfig, infrastructure::{axum_http::routers, postgres::postgres_connection::PgPoolSquad}};

use super::default_routers;


pub async fn start (config: Arc<DotEnvyConfig>, database_pool: Arc<PgPoolSquad>) -> Result<()> {

    let app = Router::new()
        .fallback(default_routers::not_found)
        .nest(
            "/api/v1/students",
            routers::students::routes(Arc::clone(&database_pool))
        )
        .route("/api/v1/health-check", get(default_routers::health_check))
        .layer(TimeoutLayer::new(Duration::from_secs(config.server.timeout)))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
                .allow_origin(Any)
        )
        .layer(TraceLayer::new_for_http());

    let address = SocketAddr::from(([0, 0, 0, 0], config.server.port));

    let listener = TcpListener::bind(address).await?;

    info!("Server is running on port {}", config.server.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal()).await?;

    Ok(())
}

async fn shutdown_signal () {

    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to initialize CTRL+C signal handler")
    };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Recieved CTRL+C signal"),
        _ = terminate => info!("Recieved terminate signal")
    }

}