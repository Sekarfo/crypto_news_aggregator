use reqwest;
use std::error::Error;
use super::models::{NewsArticle};

pub async fn fetch_news(query: &str, api_key: &str) 
    -> Result<Vec<NewsArticle>, Box<dyn Error>> 
{
    let url = format!(
        "https://newsapi.org/v2/everything?q={}&apiKey={}",
        query, api_key
    );

    let response = reqwest::get(&url).await?;
    let news_response: serde_json::Value = response.json().await?;
    
    let mut articles = Vec::new();
    
    if let Some(items) = news_response["articles"].as_array() {
        for item in items {
            let source_name = item["source"]["name"].as_str().unwrap_or("Unknown");
            
            let article = NewsArticle {
                title: item["title"].as_str().unwrap_or("No title").to_string(),
                source: source_name.to_string(),
                published_at: item["publishedAt"].as_str().unwrap_or("Unknown date").to_string(),
                summary: item["description"].as_str().map(|s| s.to_string()),
                url: item["url"].as_str().unwrap_or("#").to_string(),
            };
            
            articles.push(article);
        }
    }
    
    Ok(articles)
}