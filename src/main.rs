mod init;
mod configs;
mod routes;
mod models;

use log::{error, info};

fn main() {
    // Initialize Logger
    log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to initialize logger");
    
    // Initialize Server
    info!("Starting server...");
    let server = init::init_app();
    match server {
        Ok(_) => info!("Server started successfully"),
        Err(e) => error!("Failed to start server: {}", e)
    }
}