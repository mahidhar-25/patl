use crate::state::AppState;
use axum::{Router, routing::get};

mod health;
pub use health::health_check;

pub fn create_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(app_state)
}
