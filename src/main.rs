
use std::sync::Arc;

use my_first_rust_api::{config::config_loader, infrastructure::{axum_http::http_serve::start, postgres::postgres_connection}};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    let dotenvy_env = match config_loader::load() {
        Ok(env) => env,
        Err(error) => {
            error!("Failed to load ENV : {}", error);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded!");

    let postgres_connection_pool = match postgres_connection::establish_connection(&dotenvy_env.database.url) {
        Ok(pool) => pool,
        Err(error) => {
            error!("Failed to establish database connection : {}", error);
            std::process::exit(1);
        }
    };

    info!("Database established successfully!");

    start(Arc::new(dotenvy_env), Arc::new(postgres_connection_pool)).await.expect("Failed to start server");

}
