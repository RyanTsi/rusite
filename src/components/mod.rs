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

#[derive(Clone, PartialEq, Props)]
pub struct ListProps { 
    content: Vec<Element>,
    #[props(default = 16)]
    gap: usize,
}

#[component]
pub fn List(props: ListProps) -> Element {
    let gap = format!("gap-{}", props.gap);
    rsx! {
        div {
            "class": "flex flex-col {gap}",
            for item in props.content.iter() {
                { item },
            }
        }
    }
}