use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub date: String,
    pub summary: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoinMetadata {
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub logo: String,
    pub website: Option<String>,
    pub explorer: Option<String>,
    pub source_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombinedResponse {
    pub metadata: CoinMetadata,
    pub news: Vec<NewsArticle>,
}