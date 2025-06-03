use dioxus::prelude::*;
use lucide_dioxus::Tags;
use serde::Deserialize;

use crate::{assets::{folders_icon, left_icon, right_icon}, components::ArticleMain, models::ArticleInfo, routes::Route};

#[derive(Deserialize)]
pub struct ApiResponse<T> {
    code: u32,
    message: String,
    pub data: T,
}


fn get_article_by_id(aid: &str) -> ArticleInfo {
    // 这里应该调用 API 获取真实数据
    ArticleInfo {
        aid: aid.to_string(),
        title: format!("文章标题 {}", aid),
        summary: "这是文章摘要".to_string(),
        categories: vec!["Rust".to_string(), "Web 开发".to_string()],
        tags: vec!["Dioxus".to_string(), "前端".to_string()],
        secret: None,
        created_at: "2025-01-01".to_string(),
        updated_at: "2025-01-05".to_string(),
    }
}

#[component]
pub fn Article(aid: String, title: String, created_at: String, updated_at: String, tags: Vec<String>, categories: Vec<String>) -> Element {
    // 模拟获取文章数据（你可以替换成从 API 获取）
    // let article = use_memo(move || get_article_by_id(&aid))();
    rsx! {
        ArticleMain {
            aid: aid,
            title: title,
            created_at: created_at,
            updated_at: updated_at,
            tags: tags,
            categories: categories,
        }
        // div {
        //     class: "p-8 max-w-4xl mx-auto",
        //     // 返回按钮
        //     div {
        //         class: "mb-6",
        //         Link {
        //             to: Route::Blog {},
        //             class: "text-blue-500 hover:text-blue-700 flex items-center gap-2",
        //             left_icon { size: 4 }
        //             "返回博客"
        //         }
        //     }

        //     // 标题
        //     h1 {
        //         class: "text-4xl font-bold mb-4",
        //         "{article.title}"
        //     }

        //     // 时间信息
        //     div {
        //         class: "flex gap-4 text-gray-500 text-sm mb-6",
        //         span { "发布于：{article.created_at}" }
        //         span { "更新于：{article.updated_at}" }
        //     }

        //     // 分类和标签
        //     div {
        //         class: "flex flex-wrap gap-4 mb-6",
        //         if !article.categories.is_empty() {
        //             div {
        //                 class: "flex flex-wrap gap-2 items-center",
        //                 folders_icon { size: 5 }
        //                 for category in article.categories.iter() {
        //                     span {
        //                         class: "px-3 py-1 bg-blue-100 text-blue-700 rounded-full text-sm",
        //                         "{category}"
        //                     }
        //                 }
        //             }
        //         }
        //         if !article.tags.is_empty() {
        //             div {
        //                 class: "flex flex-wrap gap-2 items-center",
        //                 Tags { size: 24 }
        //                 for tag in article.tags.iter() {
        //                     span {
        //                         class: "px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm",
        //                         "{tag}"
        //                     }
        //                 }
        //             }
        //         }
        //     }

        //     // 正文 + 侧边栏布局
        //     div {
        //         class: "flex gap-8",
        //         // 主体内容区
        //         div {
        //             class: "w-3/4",
        //             id: "content",
        //             // 假设正文是一个 Markdown 渲染组件或静态 HTML
        //             p { "这里是正文内容……" }
        //         }

        //         // 侧边栏大纲
        //         div {
        //             class: "w-1/4 sticky top-24 self-start",
        //             SidebarOutline {}
        //         }
        //     }
        // }
    }
}

#[component]
fn SidebarOutline() -> Element {
    rsx! {
        div {
            class: "bg-white p-4 rounded shadow",
            h3 { class: "font-semibold mb-2", "文章大纲" }
            ul {
                class: "space-y-1",
                li {
                    a {
                        href: "#introduction",
                        class: "block text-sm text-gray-600 hover:text-blue-500",
                        "简介"
                    }
                }
                li {
                    a {
                        href: "#setup",
                        class: "block text-sm text-gray-600 hover:text-blue-500",
                        "环境搭建"
                    }
                }
                li {
                    a {
                        href: "#usage",
                        class: "block text-sm text-gray-600 hover:text-blue-500",
                        "使用方法"
                    }
                }
            }
        }
    }
}