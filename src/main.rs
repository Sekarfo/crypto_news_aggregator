use actix_web::{App, HttpServer, middleware};
use dotenvy::dotenv;
use std::env;
use actix_cors::Cors;

mod api;
mod models;
mod routes;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    env_logger::init();
    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    println!("Starting server at: http://{}", &addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header())
            .wrap(middleware::Logger::default())
            .service(routes::get_news)
    })
    .bind(&addr)?
    .run()
    .await
}
