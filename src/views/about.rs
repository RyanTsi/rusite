use dioxus::prelude::*;

/// About page
#[component]
pub fn About() -> Element {
    rsx! {
        div {
            "about me"
        }
    }
}