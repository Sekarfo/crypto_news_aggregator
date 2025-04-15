mod api;
mod models;
mod handler;
mod cache;

use dotenv::dotenv;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let static_files = warp::fs::dir("static");
    let routes = handler::routes()
        .or(static_files)
        .with(warp::cors().allow_any_origin());


    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());


    println!("Server running at http://localhost:{port}");
    warp::serve(routes)
        .run(([127, 0, 0, 1], port.parse().unwrap()))
        .await;
}
