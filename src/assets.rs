use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const HEADER_SVG: Asset = asset!("/assets/header.svg");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Props, PartialEq, Clone)]
pub struct IconProps {
    size: usize,
}
pub fn search_icon(iconprops: IconProps) -> Element{
    rsx!(
        svg {
            class: "-ml-0.5 size-{iconprops.size} fill-gray-600 dark:fill-gray-500",
            view_box: "0 0 16 16",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                clip_rule: "evenodd",
                d: "M9.965 11.026a5 5 0 1 1 1.06-1.06l2.755 2.754a.75.75 0 1 1-1.06 1.06l-2.755-2.754ZM10.5 7a3.5 3.5 0 1 1-7 0 3.5 3.5 0 0 1 7 0Z",
                fill_rule: "evenodd",
            }
        }
    )
}