#![allow(non_snake_case)]
use crate::services::keywords::{get_keyword, get_keywords};
use crate::views::{
    shared::Card, DetailedItemProperties, DetailedProps, SimpleItemProperties, SimpleProps,
};
use crate::CurrentDetailedObjects;
use dioxus::prelude::*;

// Set here what you want to show, id and date_created are already passed to the component
const KEYWORD_SIMPLE_HEADERS: [&str; 1] = ["keyword_name"];
const KEYWORD_DETAILED_HEADERS: [&str; 1] = ["keyword_name"];

#[component]
pub fn Keywords(cx: Scope) -> Element {
    let mut name_filter = "".to_string();
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

    let currentDetailStruct = use_shared_state::<CurrentDetailedObjects>(cx).unwrap();

    let keyword = use_future!(cx, |currentDetailStruct| async move {
        get_keyword(currentDetailStruct.read().Keyword)
            .await
            .unwrap()
    })
    .value()?;

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

    render! {
        // name_filter = "".to_string(),
        div {
            div { class: "flex flex-row items-center justify-center bg-gray-200",
                rsx! {input {
                    // we tell the component what to render

                    // and what to do when the value changes
                    // oninput: move |evt| name.set(evt.value.clone()),
                    oninput: move |evt| {
                        name_filter = evt.value.clone().to_string(); 
                        log::info!("cx.render Filter {} {}",  evt.value.clone().to_string(), name_filter);
                    },
                }}
            },

            div { class: "flex flex-row items-center justify-center bg-gray-200",
                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"simple_list",
                    model: &"Keyword",
                    headers_vec: keyword_simple_headers_vec.clone(),
                    detailed_headers_vec: keyword_detailed_headers_vec.clone(),
                    item_vec: item_vec.clone().into_iter().filter(|x| x.props.1.contains(&name_filter)).collect(),
                    detailed_item: detailed_item.clone(),
                },

                Card {
                    card_title: title,
                    card_subtitle: subtitle,
                    r#type: &"detailed_view",
                    model: &"Keyword",
                    headers_vec: keyword_simple_headers_vec,
                    detailed_headers_vec: keyword_detailed_headers_vec,
                    item_vec: item_vec,
                    detailed_item: detailed_item,
                },
            },
        },
    }
}
