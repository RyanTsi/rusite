use dioxus::prelude::*;


#[component]
pub fn Overlay(active: Signal<bool>) -> Element {
    rsx! {
        div {
            id: "overlay",
            class: "fixed inset-0 bg-black/50 min-h-screen backdrop-blur-md",
            onclick: move |_| active.set(false),
        }
    }
}

#[component]
pub fn List(content: Vec<Element>) -> Element { 
    rsx! {
        div {
            class: "flex flex-col gap-4",
            for item in content {
                { item },
            }
        }
    }
}