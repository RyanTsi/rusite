use dioxus::prelude::*;
use rusite_front_end::routes::Route;
use rusite_front_end::assets::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let active_search = use_signal(|| false);
    provide_context(active_search);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS } 
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "flex flex-col bg-gray-100 min-h-screen w-full h-full -z-50",
            Router::<Route> {}
        }
        
    }
}

