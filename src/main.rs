//use axum::Router;
//use hyper::Server;
use std::net::SocketAddr;

mod config;
mod models; // user model
mod schema;
mod services;
mod state;
mod utils; // for AppError // auth services

//mod routes; // route handlers

use config::AppConfig;
use state::AppState;

#[tokio::main]
async fn main() {
    // Load environment configuration
    let config = AppConfig::from_env();

    // Initialize application state (DB + config)
    let app_state = AppState::new(config.clone());

    // Set up the app router (you'll fill this in step 8)
    // let app = Router::new()
    //     .merge(routes::auth::routes()) // example, coming in next step
    //     .with_state(app_state);

    // Define address from config port
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("🚀 Server running on http://{}", addr);

    // Start the Axum server
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
}
