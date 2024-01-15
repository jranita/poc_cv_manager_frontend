#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::router::Route;

#[component]
pub fn PageNotFound(cx: Scope, route: Vec<String>) -> Element {
    println!("Page not found {:?}.", route);
    render! {
        h1 { "Page not found" }
        div {
            Link { to: Route::Home {}, class:"bg-yellow-300 text-white text-sm leading-6 font-medium py-2 px-3 rounded-lg", "Go to Home" }
        }
    }
}