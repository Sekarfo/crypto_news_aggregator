use super::api::fetch_news;
use super::models::NewsArticle;
use std::error::Error;

/// Business logic: get news, transform data if needed, etc.
pub async fn get_crypto_news(query: &str, api_key: &str) 
    -> Result<Vec<NewsArticle>, Box<dyn Error>> 
{
    // Potential place for caching or additional logic
    // For now, simply call the API function
    let articles = fetch_news(query, api_key).await?;
    Ok(articles)
}
