#![allow(non_snake_case)]

use crate::router::Route;
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

pub type SimpleProps<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str);
pub type DetailedProps<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a str, &'a str);


#[derive(Debug, PartialEq, Clone)]
pub struct SimpleItemProperties<'a> {
    id: usize,
    date_created: String,
    props: SimpleProps<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DetailedItemProperties<'a> {
    id: usize,
    date_created: String,
    props: DetailedProps<'a>,
}
