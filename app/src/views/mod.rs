#![allow(non_snake_case)]

use crate::router::Route;
use dioxus::prelude::*;
use dioxus::{
    core::{Element, Scope},
    core_macro::{component, render},
};
use dioxus_router::components::Link;
use crate::CurrentDetailedObjects;

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
    id: i32,
    date_created: String,
    props: SimpleProps<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DetailedItemProperties<'a> {
    id: i32,
    date_created: String,
    props: DetailedProps<'a>,
}

pub fn changeCurrentObject(cx: Scope, model: String, newId: i32) -> () {
    let currentDetailStruct = use_shared_state::<CurrentDetailedObjects>(cx).unwrap();

    match model {
        ClientCompany => currentDetailStruct.write().ClientCompany = newId,
        CV => currentDetailStruct.write().CV = newId,
        JobFunction => currentDetailStruct.write().JobFunction = newId,
        Keyword => currentDetailStruct.write().Keyword = newId,
        User => currentDetailStruct.write().User = newId,
        _ => return ()
    }
    let currentObjectId = use_shared_state::<CurrentDetailedObjects>(cx).unwrap();
    ()
    // let style = if dark_mode.read().0 {
    //     "color:white"
    // } else {
    //     ""
    // };

    // cx.render(rsx!(label {
    //     style: "{style}",
    //     "Dark Mode",
    //     input {
    //         r#type: "checkbox",
    //         oninput: move |event| {
    //             let is_enabled = event.value == "true";
    //             dark_mode.write().0 = is_enabled;
    //         },
    //     },
    // }))
}

pub fn getCurrentObject(cx: Scope, model: String) -> i32 {
    let currentDetailStruct = use_shared_state::<CurrentDetailedObjects>(cx).unwrap().read();

    match model {
        ClientCompany => currentDetailStruct.ClientCompany,
        CV => currentDetailStruct.CV,
        JobFunction => currentDetailStruct.JobFunction,
        Keyword => currentDetailStruct.Keyword,
        User => currentDetailStruct.User,
        _ => 1,
    }

}


