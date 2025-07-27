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

#[derive(Serialize)]
pub struct Comment {
    pub cid: String,
    pub uid: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct Tag {
    pub name: String,
    pub count: usize,
}

#[derive(Serialize)]
pub struct Category{
    pub name: String,
    pub count: usize,
}