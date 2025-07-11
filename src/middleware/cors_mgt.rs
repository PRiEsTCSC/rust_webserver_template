use actix_cors::Cors;
use actix_web::http::header;
use std::env;

pub fn handle_cors() -> Cors {
    let allowed_origin = env::var("FRONTEND_URL").expect("FRONTEND_URL is not set");

    Cors::default()
        .allowed_origin(&allowed_origin)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
        .supports_credentials()
}
