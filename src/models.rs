use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub date: String,
    pub summary: String,
    pub link: String,
}
