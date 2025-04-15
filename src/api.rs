use crate::models::NewsArticle;
use crate::models::CoinMetadata;
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

pub async fn fetch_metadata_coinmarketcap(symbol: &str) -> Result<CoinMetadata, Box<dyn std::error::Error>> {
    let api_key = env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY not set");

    let client = reqwest::Client::new();
    let url = format!(
        "https://pro-api.coinmarketcap.com/v2/cryptocurrency/info?symbol={}",
        symbol.to_uppercase()
    );

    let res = client
        .get(&url)
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    // data is map of { "BTC": { ... } }
    if let Some(data_map) = res.get("data").and_then(|d| d.as_object()) {
        if let Some(array) = data_map.get(&symbol.to_uppercase()).and_then(|v| v.as_array()) {
            if let Some(value) = array.first() {
                let name = value["name"].as_str().unwrap_or_default().to_string();
                let symbol = value["symbol"].as_str().unwrap_or_default().to_string();
                let description = value["description"].as_str().unwrap_or_default().to_string();
                let logo = value["logo"].as_str().unwrap_or_default().to_string();
    
                let website = value["urls"]["website"]
                    .get(0)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
    
                let explorer = value["urls"]["explorer"]
                    .get(0)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
    
                let source_code = value["urls"]["source_code"]
                    .get(0)
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
    
                return Ok(CoinMetadata {
                    name,
                    symbol,
                    description,
                    logo,
                    website,
                    explorer,
                    source_code,
                });
            }
        }
    }
    

    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Coin data not found",
    )))
}