#![allow(non_snake_case)]
use chrono::DateTime;
use dioxus::prelude::*;
use dioxus_router::components::Link;
use crate::{models::keyword, views::{shared::Card, Props, SimpleItemProperties}};
use crate::{
    models::keyword::Keyword, router::Route, services::keywords::get_keywords,
};

// Set here what you want to show, id and date_created are already passed to the component
const KEYWORD_SIMPLE_HEADERS: [&str; 1] = ["keyword_name"];

#[component]
pub fn Keywords(cx: Scope) -> Element {
    let title: String = "Keywords".to_string();
    let subtitle: String = "ttt1".to_string();
    let keyword_vec = use_future!(cx, || async move {
        get_keywords(
            999,
            0,
            "keyword_name".to_owned(),
            "ASC".to_owned(),
            "".to_owned(),
        )
        .await
        .unwrap()
    })
    .value()?;

    let keyword_simple_headers_vec: Vec<&'static str> = Vec::from(KEYWORD_SIMPLE_HEADERS);

    // Set here what you want to show, should match KEYWORD_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<Props> = vec![];
    for item in keyword_vec {
        props_vec.push(
            (&item.keyword_name, "", "", "", "")
        );
    }

    let item_vec: Vec<SimpleItemProperties> = keyword_vec.clone().iter().enumerate().map(|item| SimpleItemProperties{
        id: item.1.id,
        date_created: item.1.date_created.to_string(),
        props: props_vec[item.0],
    }).collect();

    render! {
        div {
            div { class: "flex flex-row items-center justify-center bg-gray-200",
                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"simple_list",
                    model: &"Keywords",
                    headers_vec: keyword_simple_headers_vec.clone(),
                    item_vec: item_vec.clone(),
                },

                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"detailed_view",
                    model: &"Keywords",
                    headers_vec: keyword_simple_headers_vec,
                    item_vec: item_vec,
                },
            },
        },
    }
}
