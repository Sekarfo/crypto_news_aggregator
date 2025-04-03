use serde::{Deserialize, Serialize};

/// Represents a single news article from an external API.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub published_at: String,
    pub summary: Option<String>,
    pub url: String,
}

/// Represents the structure of a response from the external API.
/// Adjust fields based on the actual API response.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub data: Vec<NewsArticle>,
    // If the API returns status codes, errors, etc., add them here.
}
