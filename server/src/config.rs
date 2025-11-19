//! Server configuration

use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Server configuration
    pub server: ServerConfig,

    /// Database configuration
    pub database: DatabaseConfig,

    /// Redis configuration
    pub redis: RedisConfig,

    /// Rate limiting configuration
    pub rate_limit: RateLimitConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,

    /// Port to bind to
    pub port: u16,

    /// Log level
    pub log_level: String,
}

impl ServerConfig {
    pub fn address(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid server address")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL
    pub url: String,

    /// Maximum number of connections
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RateLimitConfig {
    /// Free tier: requests per minute
    pub free_rpm: u32,

    /// Pro tier: requests per minute
    pub pro_rpm: u32,

    /// Business tier: requests per minute
    pub business_rpm: u32,
}

impl Config {
    /// Load configuration from environment and config files
    pub fn load() -> anyhow::Result<Self> {
        // Load .env file if it exists
        dotenvy::dotenv().ok();

        let config = config::Config::builder()
            // Default values
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .set_default("server.log_level", "info")?
            .set_default("database.max_connections", 10)?
            .set_default("rate_limit.free_rpm", 10)?
            .set_default("rate_limit.pro_rpm", 100)?
            .set_default("rate_limit.business_rpm", 1000)?
            // Environment variables override
            .add_source(config::Environment::with_prefix("TK").separator("__"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
