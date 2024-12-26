
use anyhow::{Ok, Result};
use super::{config_model::{Database, DotEnvyConfig, JWTSecret, Server}, stage::Stage};

pub fn load () -> Result<DotEnvyConfig> {

    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT").expect("Server port is invalid!").parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT").expect("Body limit is invalid").parse()?,
        timeout: std::env::var("SERVER_TIMEOUT").expect("Timeout is invalid").parse()?
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("Database URL is invalid")
    };

    Ok(DotEnvyConfig {
        server,
        database
    })

}

pub fn get_stage () -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_jwt_secret_env () -> Result<JWTSecret> {
    dotenvy::dotenv().ok();

    let secret = std::env::var("JWT_SECRET_KEY").expect("JWT secret is invalid");
    let refresh_secret = std::env::var("JWT_REFRESH_SECRET_KEY").expect("JWT refresh secret is invalid");

    Ok(JWTSecret {
        secret,
        refresh_secret
    })
}