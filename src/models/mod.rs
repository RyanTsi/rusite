use dioxus::prelude::*;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct ApiResponse<T> {
    code: u32,
    message: String,
    data: T,
}

impl<T> ApiResponse<T> {
    pub fn is_success(&self) -> bool {
        self.code == 200
    }
    pub fn data(self) -> T {
        self.data
    }
}

#[derive(Props, PartialEq, Clone, Deserialize)]
pub struct ArticleInfo {
    pub aid: String,
    title: String,
    summary: String,
    tags: Vec<String>,
    categories: Vec<String>,
    secret: Option<String>,
    created_at: String,
    updated_at: String,
}

impl ArticleInfo {
    pub fn aid(&self) -> &str {
        &self.aid
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn summary(&self) -> &str {
        &self.summary
    }
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn categories(&self) -> &Vec<String> {
        &self.categories
    }
    pub fn secret(&self) -> Option<&String> {
        self.secret.as_ref()
    }
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    pub fn updated_at(&self) -> &str {
        &self.updated_at
    }
}

pub struct Article {
    info : ArticleInfo,
    content: String,
}

impl Article {
    pub fn new(info: ArticleInfo, content: String) -> Self {
        Self { info, content }
    }

    pub fn info(&self) -> &ArticleInfo {
        &self.info
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
