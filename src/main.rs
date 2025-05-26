use dioxus::prelude::*;
use lucide_dioxus::{Moon, Sun};
use rusite_front_end::routes::Route;
use rusite_front_end::assets::*;

fn main() {
    dotenv::dotenv().ok();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_dark = use_signal(|| false);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: if is_dark() { "dark" } else { "" },
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

