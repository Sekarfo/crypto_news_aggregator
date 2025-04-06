use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub published_at: String,
    pub summary: Option<String>,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub status: String,
    pub totalResults: usize,
    pub articles: Vec<ApiArticle>,
}

#[derive(Debug, Deserialize)]
pub struct ApiArticle {
    pub source: Source,
    pub title: String,
    pub description: Option<String>,
    pub url: String,
    pub publishedAt: String,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}