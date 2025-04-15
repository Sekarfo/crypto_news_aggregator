use std::time::Duration;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use crate::models::CombinedResponse;

pub struct CacheEntry {
    pub timestamp: DateTime<Utc>,
    pub data: CombinedResponse,
}

pub struct AppCache {
    pub map: DashMap<String, CacheEntry>,
    pub ttl: Duration,
}

impl AppCache {
    pub fn new(ttl_secs: u64) -> Self {
        AppCache {
            map: DashMap::new(),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    pub fn get(&self, symbol: &str) -> Option<CombinedResponse> {
        if let Some(entry) = self.map.get(symbol) {
            let age = Utc::now().signed_duration_since(entry.timestamp);
            if age.to_std().unwrap() < self.ttl {
                return Some(entry.data.clone());
            } else {
                self.map.remove(symbol);
            }
        }
        None
    }

    pub fn set(&self, symbol: &str, data: CombinedResponse) {
        let entry = CacheEntry {
            timestamp: Utc::now(),
            data,
        };
        self.map.insert(symbol.to_string(), entry);
    }
}
