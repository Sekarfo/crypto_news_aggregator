use crate::models::NewsArticle;
use std::env;
use reqwest::Error;

pub async fn fetch_from_cryptopanic(symbol: &str) -> Result<Vec<NewsArticle>, Error> {
    let api_key = env::var("CRYPTOPANIC_API_KEY").expect("CRYPTOPANIC_API_KEY not set");

    let url = format!(
        "https://cryptopanic.com/api/v1/posts/?auth_token={}&currencies={}",
        api_key, symbol
    );

    let resp = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

    let mut articles = Vec::new();

    if let Some(results) = resp.get("results").and_then(|r| r.as_array()) {
        for item in results.iter().take(5) {
            let title = item["title"].as_str().unwrap_or_default().to_string();
            let published_at = item["published_at"].as_str().unwrap_or_default().to_string();
            let url = item["url"].as_str().unwrap_or_default().to_string();
            let source = item["source"]["title"].as_str().unwrap_or("CryptoPanic").to_string();
            let summary = item["slug"].as_str().unwrap_or("").to_string();

            articles.push(NewsArticle {
                title,
                date: published_at,
                link: url,
                source,
                summary,
            });
        }
    }

    Ok(articles)
}
