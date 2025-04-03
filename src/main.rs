use actix_web::{App, HttpServer, middleware, web};
use dotenvy::dotenv;
use std::env;

mod api;
mod models;
mod routes;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env (optional)
    dotenv().ok();
    
    // Initialize logger (optional)
    env_logger::init();
    
    // Grab a port from environment or default
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("Starting server at: http://{}", &addr);

    // Start HTTP server
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // For logging requests
            .service(routes::get_news)
    })
    .bind(&addr)?
    .run()
    .await
}
