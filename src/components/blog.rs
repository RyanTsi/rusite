use std::{cmp::{max, min}, collections::HashSet};

use async_std::task::sleep;
use dioxus::{document::eval, prelude::*};
use reqwest::get;
use crate::components::article::{ApiResponse, ArticleInfo, ArticleList};

// static SERVER = Server{
//     config: Config {
//         api_url: "https://api.github.com".to_string(),
//         api_key: "".to_string(),
//     },
// }

/// Blog page
#[component]
pub fn Blog() -> Element {
    let mut articlelist = use_signal(|| Vec::new());
    let mut max_page= use_signal(|| 1);
    let mut loading = use_signal(|| true);
    use_future(move || async move {
        articlelist.set(get("http://localhost:8000/api/v1/article/list")
        .await.unwrap().json::<ApiResponse::<Vec<ArticleInfo>>>().await.unwrap().data);
        loading.set(false);
    });
    let mut cur_page = use_signal(|| 1);
    let mut cur_articles = use_signal(|| Vec::new());
    let selected_tags: Signal<HashSet<String>> = use_signal(|| {HashSet::new()});
    let selected_categories: Signal<HashSet<String>> = use_signal(|| HashSet::new());
    let mut tags = Vec::new();
    let mut categories = Vec::new();

    for i in 1..=10 { 
        tags.push("value".to_string() + &i.to_string());
        categories.push("tutorial".to_string() + &i.to_string());
    }
    
    use_effect(move || {
        let filtered: Vec<ArticleInfo> = articlelist().iter()
        .filter(|info| {
            selected_tags().is_empty() || info.tags.iter().any(|tag| selected_tags().contains(tag))
        })
        .filter(|info| {
            selected_categories().is_empty() || info.categories.iter().any(|category| selected_categories().contains(category))
        })
        .cloned()
        .collect();
        max_page.set(max(1, (filtered.len() + 9) / 10));
        if  cur_page() > max_page() { cur_page.set(1); }
        let start = 10 * (cur_page() - 1);
        let end = min(start + 10, filtered.len());
        cur_articles.set(filtered[start..end].to_vec());
    });

    
    rsx! {
        if loading() {
            div {
                class: "flex justify-center items-center h-screen",
                "Loading..."
            }
        } else{
            div {
                id: "blog",
                class: "flex flex-row",
                div {
                    class: "flex flex-col gap-8 items-end mt-8 w-3/4",
                    for info in cur_articles() {
                        ArticleList { 
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
                div {
                    class: "w-1/4 sticky top-24 h-fit self-start",
                    Sidebar {
                        tags: tags,
                        categories: categories,
                        selected_tags:  selected_tags,
                        selected_categories: selected_categories,
                    }
                }
            }
            div {
                class: "flex justify-center mt-12 mb-8 w-full",
                for i in 1..=max_page() {
                    if i == cur_page() {
                        button {
                            class: "mx-1 px-4 py-2 bg-blue-600 text-white rounded-md",
                            "{i}"
                        }
                    } else {
                        button {
                            class: "mx-1 px-4 py-2 bg-gray-200 hover:bg-gray-300 rounded-md",
                            onclick: move |_| {
                                cur_page.set(i);
                                eval(r#"
                                    window.scrollTo({
                                        top: 0,
                                        behavior: 'smooth'
                                    });
                                "#);
                            },
                            "{i}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Sidebar(
    tags: Vec<String>,
    categories: Vec<String>,
    selected_tags: Signal<HashSet<String>>,
    selected_categories: Signal<HashSet<String>>,
) -> Element {
    rsx! {
        div {
            class: "bg-white p-4 rounded-lg shadow-md mx-4 overflow-y-auto max-h-[70vh]",
            
            // 标签部分
            h3 { class: "font-bold text-lg mb-2", "筛选标签" }
            div {
                class: "flex flex-wrap gap-2 mb-4",
                for tag in tags {
                    button {
                        class: if selected_tags.read().contains(&tag) {
                            "px-3 py-1 bg-blue-600 text-white rounded-full text-sm"
                        } else {
                            "px-3 py-1 bg-gray-200 hover:bg-gray-300 rounded-full text-sm"
                        },
                        onclick: move |_| {
                            if selected_tags.read().contains(&tag) {
                                selected_tags.write().remove(&tag);
                            } else {
                                selected_tags.write().insert(tag.clone());
                            }
                        },
                        "{tag}"
                    }
                }
            }

            // 分类部分
            h3 { class: "font-bold text-lg mb-2", "筛选分类" }
            div {
                class: "flex flex-wrap gap-2",
                for category in categories {
                    button {
                        class: if selected_categories.read().contains(&category) {
                            "px-3 py-1 bg-blue-600 text-white rounded-full text-sm"
                        } else {
                            "px-3 py-1 bg-gray-200 hover:bg-gray-300 rounded-full text-sm"
                        },
                        onclick: move |_| {
                            if selected_categories.read().contains(&category) {
                                selected_categories.write().remove(&category);
                            } else {
                                selected_categories.write().insert(category.clone());
                            }
                        },
                        "{category}"
                    }
                }
            }

            // 清除筛选按钮
            button {
                class: "mt-4 px-3 py-1 bg-red-500 text-white rounded-full text-sm w-full",
                onclick: move |_| {
                    selected_tags.write().clear();
                    selected_categories.write().clear();
                },
                "清除筛选"
            }
        }
    }
}