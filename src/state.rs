use crate::{fetch::Fetch, models::Article};

pub struct AppState {
    fetch: Fetch,
    articles: Vec<Article>,
}

impl AppState {
    pub fn new() -> Self {
        let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8000".to_string());
        let fetch = Fetch::new(&base_url);
        Self {
            fetch,
            articles:  Vec::new(),
        }
    }
    pub async fn load_articles(&mut self) {
        if let Ok(articles) = self.fetch.fetch_articles().await {
            self.articles = articles;
        }
    }
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}