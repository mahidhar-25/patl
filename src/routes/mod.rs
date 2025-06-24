use crate::state::AppState;
use axum::{Router, routing::get, routing::post};

mod health;
mod user;
pub use health::health_check;
pub use user::{login_user, register_user};

pub fn create_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .with_state(app_state)
}
