use dotenv::dotenv;
use std::env;

pub fn _load_envs() -> (String, u16) {
    dotenv().ok();

    let host = env::var("HOST")
        .expect("HOST environment variable must be set");
    
    let port = env::var("PORT")
        .expect("PORT environment variable must be set")
        .parse::<u16>()
        .expect("PORT must be a valid 16-bit unsigned integer");
    
    (host, port)
}

pub fn load_database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable must be set")
}