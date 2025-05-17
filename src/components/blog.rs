use dioxus::prelude::*;
use crate::components::article::ArticleInfo;

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    let mut article_info = Vec::new();
    article_info.push(
        ArticleInfo {
            aid: "1".to_string(),
            title: "Dioxus Router".to_string(),
            summary: "Dioxus Router is a powerful router for Dioxus applications.".to_string(),
            tags: vec!["dioxus".to_string(), "router".to_string()],
            categories: vec!["tutorial".to_string()],
            secret: None,
            created_at: "2025-5-17".to_string(),
            updated_at: "2025-5-17".to_string(),
        }
    );
    article_info.push(
        ArticleInfo {
            aid: "2".to_string(),
            title: "Dioxus Router2".to_string(),
            summary: "Dioxus Router is a powerful router for Dioxus applications.".to_string(),
            tags: vec!["dioxus".to_string(), "router".to_string()],
            categories: vec!["tutorial".to_string()],
            secret: None,
            created_at: "2025-5-17".to_string(),
            updated_at: "2025-5-17".to_string(),
        }
    );
    rsx! {
        div {
            id: "blog",
            for info in article_info {
                ArticleInfo { 
                    aid: info.aid,
                    title: info.title,
                    summary: info.summary,
                    tags: info.tags,
                    categories: info.categories,
                    secret: info.secret,
                    created_at: info.created_at,
                    updated_at: info.updated_at,
                }
            }
        }
    }
}

#[component]
pub fn Article(aid: String) -> Element { 
    rsx!(

    )
}