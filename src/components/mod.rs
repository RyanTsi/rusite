use dioxus::prelude::*;
use lucide_dioxus::{ChevronRight, Folders, Tags};
use reqwest::get;

use crate::{models::ArticleInfo, routes::Route, views::article::ApiResponse};


#[component]
pub fn Overlay(active: Signal<bool>) -> Element {
    rsx! {
        div {
            id: "overlay",
            class: "fixed inset-0 bg-black/50 min-h-screen backdrop-blur-md",
            onclick: move |_| active.set(false),
        }
    }
}

type ArticleCardProps<'a> = ArticleInfo;

#[component]
pub fn ArticleCard(props: ArticleCardProps) -> Element {
    rsx! {
        div {
            class: "p-8 bg-white rounded-lg shadow-md hover:shadow-lg transition-shadow duration-200 w-2/3",
            div {
                class: "flex flex-row justify-between",
                h1 {
                    id: "title",
                    class : "text-3xl font-bold",
                    "{props.title}"
                }
                p {
                    id: "updated time",
                    class: "text-gray-400 text-sm",
                    "{props.updated_at}"
                }
            }
            div {
                id: "summary",
                class: "my-4",
                p {
                    class: "text-gray-600",
                    "{props.summary}"
                }
            }
            div { 
                class: "flex flex-row gap-4 justify-between items-center",
                div {
                    class: "flex flex-row gap-2 items-center",
                    // folders_icon { size: 5 }
                    Folders { size: 24 }
                    div {
                        class: "flex flex-wrap gap-2",
                        for category in props.categories.iter() {
                            span {
                                class: "px-3 py-1 bg-blue-100 text-blue-700 rounded-full text-sm",
                                "{category}"
                            }
                        }
                    }
                    Tags { size: 24 }
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
                div {
                    Link {
                        to: Route::Article { aid: props.aid, title: props.title, created_at: props.created_at, updated_at: props.updated_at, tags: props.tags, categories: props.categories },
                        class: "block",
                        // 原来的内容保持不变
                        if let Some(str) = props.secret {
                            div {
                                class: "text-sm font-semibold text-red-500 mb-2",
                                "🔒 私密文章"
                            }
                        }
                        div {
                            class: "text-gray-600 hover:text-blue-400 flex flex-row items-center",
                            "Read more"
                            ChevronRight { size: 24 }
                        }

                    }
                }
            }
        }
    }
}

#[component]
pub fn ArticleMain(aid: String, title: String, created_at: String, updated_at: String, tags: Vec<String>, categories: Vec<String>) -> Element {
    let mut content = use_signal(|| String::new());
    use_future(move || {
    let value = aid.clone();
    async move {
        let a = get(format!("http://111.231.136.180:8000/api/v1/article/{}/content/path", value.clone())).await.unwrap()
            .json::<ApiResponse::<String>>().await.unwrap().data;
        content.set(
            get(format!("http://111.231.136.180:8000{}", a) ).await.unwrap().text().await.unwrap()
        )
        }
    }
    );
    rsx! {
        div {
            class: "flex flex-col items-center p-8 bg-white rounded-lg shadow-md",
            { content }
        }
    }
}