use std::net::SocketAddr;
use tokio::net::TcpListener;

mod config;
mod models;
mod routes;
mod schema;
mod services;
mod state;
mod utils;

//specific function import
use crate::config::get_config;
use crate::state::AppState;
use routes::create_routes;

/// The main entry point for the server.
///
/// 1. Load environment configuration.
/// 2. Initialize application state.
/// 3. Create a router.
/// 4. Define the address for the server.
/// 5. Bind the server to the address.
/// 6. Serve the app.
#[tokio::main]
async fn main() {
    // Load environment configuration
    let config = get_config(); // this function loads the configuration from environment variables
    // Initialize application state
    let app_state = AppState::new(config.clone()); // this function creates the application state with the loaded configuration

    // Create router
    let app = create_routes(app_state); // this function creates the router with the application state and app state is passed so that it will attach to all router we can access them easily

    // Define address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    // Serve the app
    axum::serve(listener, app).await.unwrap();
}
