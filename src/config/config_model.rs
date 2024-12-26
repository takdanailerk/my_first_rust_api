
// DotEnvy config struct for dotenvy configuration at start
#[derive(Debug, Clone)]
pub struct DotEnvyConfig {
    pub server: Server,
    pub database: Database
}

// Server struct for DotEnvy config
#[derive(Debug, Clone)]
pub struct Server {
    pub port: u16,
    pub body_limit: u64,
    pub timeout: u64
}

// Database struct for DotEnvy config
#[derive(Debug, Clone)]
pub struct Database {
    pub url: String
}

#[derive(Debug, Clone)]
pub struct JWTSecret {
    pub secret: String,
    pub refresh_secret: String
}