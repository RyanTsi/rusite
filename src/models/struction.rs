use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct ArticleInfo {
    pub title: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub summary: String,
    pub secret: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub aid: String,
}