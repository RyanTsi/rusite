use dioxus::prelude::*;
use lucide_dioxus::{CircleUserRound, Search, User};
use crate::assets::*;
use crate::components::Overlay;
use crate::routes::Route;


/// navbar component.
#[component]
pub fn Navbar() -> Element {
    let active_search = use_signal(|| false);
    let input_content = use_signal(|| String::new());
    rsx! {
        header {
            id: "navbar",
            class: "sticky top-0 flex items-center justify-between p-4 bg-white/80 backdrop-blur-sm shadow-sm z-30",
            // Left side
            div {
                class: "flex items-center space-x-12",
                // Site name and icon
                div {
                    class: "flex items-center",
                    img { src: FAVICON, class: "h-8 w-8 mr-2" }
                    span { class: "font-bold text-lg", "RUSITE" }
                }
                Routerlinks {}
            }
            // Right side
            div {
                class: "flex items-center space-x-12",
                Searchbutton { active_search: active_search },
                UserInfo {},
            }
        }
        if active_search() {
            Search_box {
                active_search: active_search,
                input_content: input_content,
            }
        }
        Outlet::<Route> {}
    }
}

#[component]
fn Searchbutton(active_search: Signal<bool> ) -> Element {
    rsx! {
        button {
            class: "inline-flex items-center min-w-48 gap-3 rounded-full bg-gray-950/2 px-2 py-1 inset-ring inset-ring-gray-950/8 dark:bg-white/5 dark:inset-ring-white/2",
            r#type: "button",
            onclick: move |_| {
                active_search.set(true);
            },
            Search { size: 18 }
            kbd {
                class: "font-sans text-gray-500 dark:text-gray-400 pr-3",
                "Search Ctrl+K"
            }
        }
    }
}

#[component]
fn Routerlinks() -> Element {
    rsx!(
        div {
            id: "links",
            class: "space-x-8",
            Link {
                to: Route::Home {},
                class: "text-gray-700 hover:text-blue-500",
                "Home"
            }
            Link {
                to: Route::Blog {},
                class: "text-gray-700 hover:text-blue-500",
                "Blog"
            }
            Link {
                to: Route::About {},
                class: "text-gray-700 hover:text-blue-500",
                "About"
            }
        }
    )
}

#[component]
fn UserInfo() -> Element {
    rsx! {
        div {
            class: "relative group",
            // img {
            //     class: "h-8 w-8 cursor-pointer rounded-full",
            //     src: "https://avatars.githubusercontent.com/u/97720243?s=400&u=5de211300fe16f6549f2c065770cdfceb7fe69be&v=4",
            //     alt: "User Icon",
            // },
            CircleUserRound { }
            div {
                class: "absolute right-0 mt-2 w-40 bg-blue-100 rounded-md shadow-lg opacity-0 group-hover:opacity-100 transition-opacity duration-200 z-10",
                ul {
                    class: "py-1",
                    li {
                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                        "Profile"
                    }
                    li {
                        class: "block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100",
                        "Logout"
                    }
                }
            }
        }
    }
}

#[component]
fn Search_box(active_search: Signal<bool>, input_content: Signal<String>) -> Element{
    rsx! {
        div {
            id: "search panel",
            class: "fixed inset-0 items-center justify-center z-50",
            Overlay { active: active_search }
            div { 
                id: "panel",
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