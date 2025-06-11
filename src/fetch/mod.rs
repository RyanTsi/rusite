use reqwest::Client;
use std::{error::Error, sync::Arc};

use crate::models::{ApiResponse, Article, ArticleInfo};

pub struct Fetch {
    client: Arc<Client>,
    base_url: String,
}

impl Fetch {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Arc::new(Client::new()),
            base_url: base_url.to_string(),
        }
    }

    pub async fn fetch_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.get(&url).send().await?.json::<ApiResponse::<T>>().await?;
        if response.is_success() {
            Ok(response.data())
        } else {
            Err("fetch error".into())
        }
    }

    pub async fn fetch_text(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}{}", self.base_url, path);
        let response = self.client.get(&url).send().await?;
        Ok(response.text().await?)
    }

    pub async fn fetch_articles_info(&self) -> Result<Vec<ArticleInfo>, Box<dyn Error>> {
        self.fetch_json("/article/list").await
    }

    pub async fn fetch_article_content(&self, aid: &str) -> Result<String, Box<dyn Error>> {
        let path: String = self.fetch_json(&format!("/article/{}/content/path", aid)).await?;
        self.fetch_text(&path).await
    }

    pub async fn fetch_articles(&self) -> Result<Vec<Article>, Box<dyn Error>> {
        let articles_info = self.fetch_articles_info().await?;
        let mut articles = Vec::new();
        for info in articles_info {
            let content = self.fetch_article_content(&info.aid()).await?;
            articles.push(Article::new(info, content));
        }
        Ok(articles)
    }
}