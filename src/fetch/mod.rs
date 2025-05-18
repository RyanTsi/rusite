use dioxus::hooks::{use_future, UseFuture};
use reqwest::get;

struct Config {
    pub api_url: String,
    pub api_key: String,
}

struct Sever {
    pub config: Config,
}


impl Sever {
    fn new(config: Config) -> Self {
        Sever { config }
    }
    pub fn fetch_articles_info(&'static self) -> UseFuture {
        let a = use_future(|| async {
            let url = format!("{}/articles", &self.config.api_url.clone());
            let res = get(url).await.unwrap();
            res.text().await.unwrap()
        });
        a
    }
}