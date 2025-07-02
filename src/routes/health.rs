use actix_web::{web, get, HttpResponse, Responder};

#[get("/health")]
async fn server_check() -> impl Responder {
    HttpResponse::Ok().body("Server is healthy")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(server_check);
}