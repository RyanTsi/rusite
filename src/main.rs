use dioxus::prelude::*;
use rusite_front_end::routes::Route;
use rusite_front_end::assets::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut active_search = use_signal(|| false);
    let mut input_content = use_signal(|| String::new());
    provide_context(active_search);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS } 
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "flex flex-col bg-gray-100 min-h-screen w-full h-screen",
            Router::<Route> {}
        }
        if active_search() {
            div {
                id: "search panel",
                class: "fixed inset-0 items-center justify-center",
                div {
                    id:"overlay",
                    class: "fixed inset-0 z-50 bg-black/50 min-h-screen backdrop-blur-md",
                    onclick: move |_| active_search.set(false),
                }
                div { 
                    id: "search-panel",
                    class: "fixed inset-x-0 mx-auto z-100 top-1/6 max-w-2xl items-center justify-center bg-white min-h-1/2",
                    div {
                        input {
                            class: "w-full rounded-full bg-gray-100 px-4 py-2 text-gray-800 dark:bg-gray-800 dark:text-gray-200",
                            placeholder: "Type here to search...",
                            r#type: "text",
                            autocomplete: "off",
                            value: "{input_content}",
                            onmounted: move |cx: Event<MountedData>| {
                                let _ = cx.data().set_focus(true);
                            },
                            onkeydown: move |evt| {
                                if evt.key() == Key::Escape {
                                    active_search.set(false);
                                }
                            },
                            oninput: move |evt| {
                                input_content.set(evt.value().clone());
                            },
                        }
                    }
                }
            }
        }
    }
}
