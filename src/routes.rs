use dioxus::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::home::Home;
use crate::components::blog::Blog;
use crate::components::article::Article;
use crate::components::about::About;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    #[route("/blog/article/:aid")]
    Article { aid: String },
    #[route("/about")]
    About {},
}