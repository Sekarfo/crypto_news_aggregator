use warp::Filter;
use crate::api::{fetch_from_cryptopanic, fetch_metadata_coinmarketcap};
use crate::models::CombinedResponse;
use once_cell::sync::Lazy;
use crate::cache::AppCache;

static CACHE: Lazy<AppCache> = Lazy::new(|| AppCache::new(600));

pub async fn news_handler(symbol: String) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Some(cached) = CACHE.get(&symbol) {
        return Ok(warp::reply::json(&cached));
    }
    
    // If not cached, fetch from APIs
    let news = fetch_from_cryptopanic(&symbol).await;
    let meta = fetch_metadata_coinmarketcap(&symbol).await;

    

    match (news, meta) {
        (Ok(news_articles), Ok(metadata)) => {
            let combined = CombinedResponse {
                metadata,
                news: news_articles,
            };
            CACHE.set(&symbol, combined.clone());
            Ok(warp::reply::json(&combined))
        },
        _ => Err(warp::reject::not_found()),
    }
}

pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("news")
        .and(warp::get())
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and_then(|params: std::collections::HashMap<String, String>| {
            let symbol = params.get("symbol").cloned().unwrap_or_default();
            news_handler(symbol)
        })
}
