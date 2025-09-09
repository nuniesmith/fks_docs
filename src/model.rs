use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocMeta {
    pub path: String,
    pub name: String,
    pub ext: String,
    pub size: u64,
    pub modified: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocContent {
    pub meta: DocMeta,
    pub etag: String,
    pub content: String,
    pub rendered_html: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResultEntry {
    pub path: String,
    pub score: u32,
    pub line_snippets: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    pub query: String,
    pub total_results: usize,
    pub results: Vec<SearchResultEntry>,
    pub took_ms: u128,
}
