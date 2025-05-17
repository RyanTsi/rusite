use dioxus::prelude::*;
use crate::assets::*;

#[component]
fn Hero() -> Element {
    rsx! {
        div {
            img { src: HEADER_SVG, id: "header" }
        }

    }
}

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col justify-center items-center min-h-3/4",
            div { 
                id: "title",
                class: "text-6xl font-bold",
                "Rusite"
            }
            div {
                id: "subtitle",
                class: "text-xl",
                "A Rust framework for building user interfaces"
            }
        }
    }
}