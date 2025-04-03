use actix_web::{get, web, HttpResponse, Responder};
use std::env;
use super::service;

/// Handle a GET request for /news/{query}
/// Example: GET /news/bitcoin
#[get("/news/{query}")]
async fn get_news(path: web::Path<String>) -> impl Responder {
    let query = path.into_inner();

    // We can load from .env or any config method
    let api_key = env::var("NEWS_API_KEY").unwrap_or_else(|_| "DUMMY_KEY".to_string());

    match service::get_crypto_news(&query, &api_key).await {
        Ok(articles) => HttpResponse::Ok().json(articles),
        Err(e) => {
            eprintln!("Error fetching news: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch news")
        }
    }
}
