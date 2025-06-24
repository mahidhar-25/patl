use dotenvy::dotenv;
use std::env;

/// App configuration loaded from environment.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub jwt_expiration: i64,
}

impl AppConfig {
    /// Load config from `.env` or environment variables.
    pub fn from_env() -> Self {
        dotenv().ok(); // Load from .env if exists

        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("PORT must be a number"),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .expect("JWT_EXPIRATION must be a number"),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::from_env()
    }
}
/// Get the application configuration.
////// # Returns
/// * `AppConfig` - The loaded configuration.
pub fn get_config() -> AppConfig {
    AppConfig::default()
}
