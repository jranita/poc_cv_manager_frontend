use crate::{router::Route, Props};
use dioxus::prelude::*;
use dioxus::{
    core::{Element, Scope},
    core_macro::{component, render},
};
use dioxus_router::components::Link;

pub mod navbar;
pub mod home;
pub mod keywords;
pub mod client_companies;
pub mod job_functions;
pub mod cvs;
pub mod users;
pub mod page404;
pub mod shared;

#[component]
pub fn HomeLink(cx: Scope) -> Element {
    render! {
        Link { to: Route::Home {}, "Home" },
    }
}
