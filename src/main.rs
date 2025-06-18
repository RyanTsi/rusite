use std::error::Error;

use dioxus::html::samp;
use dioxus::prelude::*;
use lucide_dioxus::{Moon, Sun};
use rusite_front_end::fetch::Fetch;
use rusite_front_end::models::Article;
use rusite_front_end::routes::Route;
use rusite_front_end::assets::*;
use rusite_front_end::state::AppState;

fn main() {
    dotenv::dotenv().ok();
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    let mut state = use_signal(|| AppState::new());
    use_future(move || async move {
        state.write().load_articles().await;
    });
    
    let mut is_dark = use_signal(|| false);
    provide_context(state);
    let a = state.read().articles().len();
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: if is_dark() { "dark" } else { "" },
            "{a}"
            div {
                class: "flex flex-col bg-gray-100 min-h-screen w-full h-full -z-50 dark:bg-blue-800",
                Router::<Route> {}
            }
            div {
                class: "fixed bottom-8 right-8 p-2 shadow-sm bg-gray-400 rounded-xl",
                onclick: move |_| {
                    is_dark.set(!is_dark());
                },
                if is_dark() {
                    Moon {}
                } else {
                    Sun {}
                }
            }
        }
    }
}

