use dioxus::prelude::*;
use crate::views::navbar::Navbar;
use crate::views::home::Home;
use crate::views::blog::Blog;
use crate::views::article::Article;
use crate::views::about::About;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog")]
    Blog {},
    // #[route("/blog/article/:aid")]
    // Article { aid: String, title: String, created_at: String, updated_at: String, tags: Vec<String>, categories: Vec<String> },
    #[route("/about")]
    About {},
}