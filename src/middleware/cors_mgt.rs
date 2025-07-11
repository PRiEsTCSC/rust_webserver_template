use actix_cors::Cors;
use std::env;

pub fn handle_cors() -> Cors {
    dotenv::dotenv().ok();
    let mut allowed_origin = env::var("FRONTEND_URL").expect("FRONTEND_URL is not set");

    }
    Cors::permissive
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
<<<<<<< HEAD
     // .allowed_origin(&allowed_origin)
=======
>>>>>>> 42757935330962dcda83f603efb223dd589a3de0
