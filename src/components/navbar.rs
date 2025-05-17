use dioxus::prelude::*;
use crate::assets::{search_icon, *};
use crate::routes::Route;


/// Shared navbar component.
#[component]
pub fn Navbar(active_search: Signal<bool>) -> Element {
    rsx! {
        div {
            id: "navbar",
            class: "sticky top-0 flex items-center justify-between p-4 bg-white/80 backdrop-blur-sm shadow-sm",
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
                // Search box
                Searchbutton { active_search: active_search },
                // User icon with dropdown
                UserInfo {},
            }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn Searchbutton( active_search: Signal<bool> ) -> Element {
    rsx! {
        button {
            class: "inline-flex items-center min-w-48 gap-3 rounded-full bg-gray-950/2 px-2 py-1 inset-ring inset-ring-gray-950/8 dark:bg-white/5 dark:inset-ring-white/2",
            r#type: "button",
            onclick: move |_| {
                active_search.set(true);
            },
            search_icon { size: 4 }
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
                to: Route::Blog { id: 1 },
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
            img {
                class: "h-8 w-8 cursor-pointer rounded-full",
                src: "https://avatars.githubusercontent.com/u/97720243?s=400&u=5de211300fe16f6549f2c065770cdfceb7fe69be&v=4",
                alt: "User Icon",
            },
            // Dropdown menu (hidden by default, shown on hover)
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