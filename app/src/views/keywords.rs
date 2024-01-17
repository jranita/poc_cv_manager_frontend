use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{router::Route, services::keywords::get_keywords};

#[component]
pub fn Keyword(cx: Scope, id: i32) -> Element {
    render! {
        Link { to: Route::Home {}, "Home" },
        "Keyword {id}"
    }
}

#[component]
pub fn Keywords(cx: Scope) -> Element {
    let aaa = use_future!(cx, || async move {get_keywords(999, 0, "id".to_owned(), "DESC".to_owned(), "mech".to_owned()).await.unwrap()});
    render! {
        Link { to: Route::Home {}, "Home" },
        format!("Keywords List +{:?}+", "aaa")
    }
}
