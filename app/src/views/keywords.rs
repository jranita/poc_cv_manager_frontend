#![allow(non_snake_case)]
use crate::services::keywords::{get_keyword, get_keywords};
use crate::views::{
    shared::Card, DetailedItemProperties, DetailedProps, SimpleItemProperties, SimpleProps,
};
use dioxus::prelude::*;

// Set here what you want to show, id and date_created are already passed to the component
const KEYWORD_SIMPLE_HEADERS: [&str; 1] = ["keyword_name"];
const KEYWORD_DETAILED_HEADERS: [&str; 1] = ["keyword_name"];

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

    let keyword = use_future!(cx, || async move { get_keyword(1).await.unwrap() }).value()?;

    let keyword_simple_headers_vec: Vec<&'static str> = Vec::from(KEYWORD_SIMPLE_HEADERS);
    let keyword_detailed_headers_vec: Vec<&'static str> = Vec::from(KEYWORD_DETAILED_HEADERS);

    // Set here what you want to show, should match KEYWORD_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<SimpleProps> = vec![];
    for item in keyword_vec {
        props_vec.push((&item.keyword_name, "", "", "", ""));
    }

    let item_vec: Vec<SimpleItemProperties> = keyword_vec
        .clone()
        .iter()
        .enumerate()
        .map(|item| SimpleItemProperties {
            id: item.1.id,
            date_created: item.1.date_created.to_string(),
            props: props_vec[item.0],
        })
        .collect();

    let detailed_props: DetailedProps = (&keyword.keyword_name, "", "", "", "", "", "", "");

    let detailed_item: DetailedItemProperties = DetailedItemProperties {
        id: keyword.id,
        date_created: keyword.date_created.to_string(),
        props: detailed_props,
    };
    // end TODO

    fn click_callback(id: u32) {
        log::info!("click_callback: {id}");
    }

    render! {
        div {
            div { class: "flex flex-row items-center justify-center bg-gray-200",
                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"simple_list",
                    model: &"Keywords",
                    headers_vec: keyword_simple_headers_vec.clone(),
                    detailed_headers_vec: keyword_detailed_headers_vec.clone(),
                    item_vec: item_vec.clone(),
                    detailed_item: detailed_item.clone(),
                    on_click: click_callback,
                },

                Card {
                    card_title: title,
                    card_subtitle: subtitle,
                    r#type: &"detailed_view",
                    model: &"Keywords",
                    headers_vec: keyword_simple_headers_vec,
                    detailed_headers_vec: keyword_detailed_headers_vec,
                    item_vec: item_vec,
                    detailed_item: detailed_item,
                    on_click: click_callback,
                },
            },
        },
    }
}
