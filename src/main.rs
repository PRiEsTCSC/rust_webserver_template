mod init;
mod configs;
mod routes;
mod models;
mod middleware;

{% if include_logging == "true" %}

use log::{error, info};

fn main() {
    {% if include_logging == "true" %}
    // Initialize Logger
    log4rs::init_file("log4rs.yaml", Default::default()).expect("Failed to initialize logger");
    {% endif %}
    
    // Initialize Server
    info!("Starting server...");
    let server = init::init_app();
    match server {
        Ok(_) => info!("Server started successfully"),
        Err(e) => error!("Failed to start server: {}", e)
    }
}