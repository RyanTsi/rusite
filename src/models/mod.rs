use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Props, PartialEq, Clone, Deserialize)]
pub struct ArticleInfo {
    pub aid: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub secret: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

