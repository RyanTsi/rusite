use dioxus::prelude::*;

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const HEADER_SVG: Asset = asset!("/assets/header.svg");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const BACKGROUND_IMG: Asset = asset!("/assets/bg.jpg");

#[derive(Props, PartialEq, Clone)]
pub struct IconProps {
    size: usize,
}

impl IconProps {
    fn size(&self) -> &str {
        match self.size {
            4 => "size-4",
            5 => "size-5",
            6 => "size-6",
            7 => "size-7",
            8 => "size-8",
            9 => "size-9",
            10 => "size-10",
            11 => "size-11",
            12 => "size-12",
            _  =>  "size-6",
        }
    }
}


pub fn folders_icon(props: IconProps) -> Element {
    let size = props.size();
    rsx!(
        svg {  
            class: "-ml-0.5 {size} fill-gray-600 dark:fill-gray-500",
            view_box: "0 0 576 512",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M147.8 192H480V144C480 117.5 458.5 96 432 96h-160l-64-64h-160C21.49 32 0 53.49 0 80v328.4l90.54-181.1C101.4 205.6 123.4 192 147.8 192zM543.1 224H147.8C135.7 224 124.6 230.8 119.2 241.7L0 480h447.1c12.12 0 23.2-6.852 28.62-17.69l96-192C583.2 249 567.7 224 543.1 224z",
            }
        }
    )
}

pub fn right_icon(props: IconProps) -> Element {
    let size = props.size();
    rsx!(
        svg {  
            class: "-ml-0.5 {size} fill-current dark:fill-current",
            view_box: "0 0 576 512",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M64 448c-8.188 0-16.38-3.125-22.62-9.375c-12.5-12.5-12.5-32.75 0-45.25L178.8 256L41.38 118.6c-12.5-12.5-12.5-32.75 0-45.25s32.75-12.5 45.25 0l160 160c12.5 12.5 12.5 32.75 0 45.25l-160 160C80.38 444.9 72.19 448 64 448z",
            }
        }
    )
}
pub fn left_icon(props: IconProps) -> Element {
    let size = props.size();
    rsx!(
        svg {  
            class: "-ml-0.5 {size} fill-current dark:fill-current",
            view_box: "0 0 576 512",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M192 448c-8.188 0-16.38-3.125-22.62-9.375l-160-160c-12.5-12.5-12.5-32.75 0-45.25l160-160c12.5-12.5 32.75-12.5 45.25 0s12.5 32.75 0 45.25L77.25 256l137.4 137.4c12.5 12.5 12.5 32.75 0 45.25C208.4 444.9 200.2 448 192 448z",
            }
        }
    )
}

pub fn user_icon(props: IconProps) -> Element {
    let size = props.size;
    rsx! {
        svg {
            class: "-ml-0.5 {size} fill-gray-600 dark:fill-gray-500",
            view_box: "0 0 576 512",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M256 0C114.6 0 0 114.6 0 256s114.6 256 256 256s256-114.6 256-256S397.4 0 256 0zM256 128c39.77 0 72 32.24 72 72S295.8 272 256 272c-39.76 0-72-32.24-72-72S216.2 128 256 128zM256 448c-52.93 0-100.9-21.53-135.7-56.29C136.5 349.9 176.5 320 224 320h64c47.54 0 87.54 29.88 103.7 71.71C356.9 426.5 308.9 448 256 448z",
            }
        }
    }
}