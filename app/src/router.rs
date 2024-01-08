// use crate::views::{header::AuthHeader, home::Home, login::Login, not_found::NotFound};
use crate::views::{home::Home, blog::Blog};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

// #[derive(Routable, Clone)]
// pub enum Route {
//     #[layout(AuthHeader)]
//     #[route("/")]
//     Home {},

//     // https://dioxuslabs.com/learn/0.4/router/reference/routes#query-segments
//     #[route("/login?:query_string")]
//     Login { query_string: String },
//     #[end_layout]
//     #[route("/:..route")]
//     NotFound { route: Vec<String> },
// }

#[derive(Clone, Routable, Debug, PartialEq)]
pub(crate) enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}