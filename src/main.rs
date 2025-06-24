use std::net::SocketAddr;
use tokio::net::TcpListener;

mod config;
mod models;
mod routes;
mod schema;
mod services;
mod state;
mod utils;

use crate::config::get_config;
use crate::state::AppState;
use routes::create_routes;

#[tokio::main]
async fn main() {
    // Load environment configuration
    let config = get_config();
    // Initialize application state
    let app_state = AppState::new(config.clone());

    // Create router
    let app = create_routes(app_state);

    // Define address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    // Serve the app
    axum::serve(listener, app).await.unwrap();
}
