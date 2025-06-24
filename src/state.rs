use crate::config::AppConfig;
use diesel::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};

/// A type alias for an SQLite connection pool.
pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

/// Shared application state injected into handlers.
#[derive(Clone)]
pub struct AppState {
    /// The SQLite connection pool.
    pub db_pool: SqlitePool,
    /// Application configuration.
    pub config: AppConfig,
}

impl AppState {
    /// Create a new application state with the given database pool and configuration.
    ///
    /// # Arguments
    /// * `db_pool` - The SQLite connection pool.
    /// * `config` - The application configuration.
    ///
    /// # Returns
    /// * `AppState` - The initialized application state.
    pub fn new(config: AppConfig) -> Self {
        let manager = ConnectionManager::<SqliteConnection>::new(config.database_url.clone());
        let db_pool = Pool::builder()
            .max_size(10) // Set a maximum pool size
            .build(manager)
            .expect("Failed to create database connection pool");
        Self { db_pool, config }
    }
}
