use reqwest;
use std::error::Error;
use super::models::{ApiResponse, NewsArticle};

/// Fetch news articles by keyword or symbol using an external API.
pub async fn fetch_news(query: &str, api_key: &str) 
    -> Result<Vec<NewsArticle>, Box<dyn Error>> 
{
    // Build your API URL; adjust for your chosen data provider
    let url = format!(
        "https://api.example.com/news?query={}&apiKey={}",
        query, api_key
    );

    // Send request and deserialize into ApiResponse
    let resp = reqwest::get(&url).await?.json::<ApiResponse>().await?;
    Ok(resp.data)
}
