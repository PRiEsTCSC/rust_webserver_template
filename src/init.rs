use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{self, middleware::Logger, App, HttpServer};
use actix_web::web::Data;
use crate::configs::env_load::_load_envs as load_envs;
use crate::models::database::DatabaseConnection;
use crate::routes;
{% if include_cors == "true" %}
use crate::middleware;
{% endif %}

#[actix_web::main]
pub async fn init_app() -> std::io::Result<()> {
    // Setting up the Governor
    let governor = GovernorConfigBuilder::default()
        .seconds_per_request(30)
        .burst_size(30)
        .finish()
        .unwrap();
    
    let db = DatabaseConnection::new()
        .await
        .expect("Failed to connect to database");
    
    // Start the server
    HttpServer::new(
        move || {
            App::new()
                .wrap(Logger::default())
                .wrap(Governor::new(&governor))
                {% if include_cors == "true" %}
                .wrap(middleware::cors_mgt::handle_cors())
                {% endif %}
                .app_data(Data::new(db.clone()))
                .configure(routes::init_routes)
        }
    )
    .workers(4)
    .bind((load_envs().0, load_envs().1))?
    .run()
    .await
}