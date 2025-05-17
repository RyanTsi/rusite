use dioxus::prelude::*;

use crate::routes::Route;

#[derive(Props, PartialEq, Clone)]
pub struct ArticleInfo {
    pub aid: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
    pub secret: Option<bool>,
    pub created_at: String,
    pub updated_at: String,
}

#[component]
pub fn ArticleInfo(props: ArticleInfo) -> Element {
    rsx! {
        div {
            class: "p-6 bg-white rounded-lg shadow-md mb-6 hover:shadow-lg transition-shadow duration-200 cursor-pointer",
            onclick: move |_| {
                // 可以在这里做其他逻辑处理，比如记录日志、预加载等
                // 实际跳转由 Link 处理
            },
            Link {
                to: Route::Article { aid: props.aid },
                class: "block",
                // 原来的内容保持不变
                if let Some(true) = props.secret {
                    div {
                        class: "text-sm font-semibold text-red-500 mb-2",
                        "🔒 私密文章"
                    }
                }
                h1 {
                    class: "text-3xl font-bold text-gray-800 mb-3",
                    "{props.title}"
                }
                p {
                    class: "text-gray-600 mb-4",
                    "{props.summary}"
                }
                div {
                    class: "flex flex-wrap gap-2 mb-4",
                    for category in props.categories.iter() {
                        span {
                            class: "px-3 py-1 bg-blue-100 text-blue-700 rounded-full text-sm",
                            "{category}"
                        }
                    }
                }
                div {
                    class: "flex flex-wrap gap-2",
                    for tag in props.tags.iter() {
                        span {
                            class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
                            "{tag}"
                        }
                    }
                }
            }
        }
    }
}