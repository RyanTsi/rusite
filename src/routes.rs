use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::home::Home;
use crate::components::blog::Blog;
use crate::components::article::Article;
use crate::components::about::About;

#[component]
fn Layout(children: Element) -> Element {
    let activei_search = use_context::<Signal<bool>>();
    rsx! {
        Navbar { active_search: activei_search }
        div{
            { children }
        }
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    #[route("/blog/article/:aid")]
    Article { aid: String },
    #[route("/about")]
    About {},
}