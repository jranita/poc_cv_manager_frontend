#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{
    services::cvs::{get_cvs, get_cv},
    views::{shared::Card, SimpleProps, DetailedProps, SimpleItemProperties, DetailedItemProperties}
};
use crate::CurrentDetailedObjects;

// Set here what you want to show, id and date_created are already passed to the component
const CV_SIMPLE_HEADERS: [&str; 1] = ["cv_name"];
const CV_DETAILED_HEADERS: [&str; 2] = ["cv_name", "date_created"];

#[component]
pub fn Cvs(cx: Scope) -> Element {
    let title: String = "CVs".to_string();
    let subtitle: String = "ttt2".to_string();
    let cv_vec = use_future!(cx, || async move {
        get_cvs(
            999,
            0,
            "cv_name".to_owned(),
            "ASC".to_owned(),
            "".to_owned(),
        )
        .await
        .unwrap()
    })
    .value()?;

    let currentDetailStruct = use_shared_state::<CurrentDetailedObjects>(cx).unwrap();

    let cv = use_future!(cx, |currentDetailStruct| async move {
        get_cv(currentDetailStruct.read().CV)
        .await
        .unwrap()
    })
    .value()?;

    let cv_simple_headers_vec: Vec<&'static str> = Vec::from(CV_SIMPLE_HEADERS);
    let cv_detailed_headers_vec: Vec<&'static str> = Vec::from(CV_DETAILED_HEADERS);

    // Set here what you want to show, should match CLIENT_COMPANY_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<SimpleProps> = vec![];
    for item in cv_vec {
        props_vec.push(
            (&item.cv_name, "", "", "", "")
        );
    }

    let item_vec: Vec<SimpleItemProperties> = cv_vec.clone().iter().enumerate().map(|item| SimpleItemProperties{
        id: item.1.id,
        date_created: item.1.date_created.to_string(),
        props: props_vec[item.0],
    }).collect();

    let detailed_props: DetailedProps = (&cv.cv_name, "", "", "", "", "", "", "");

    let detailed_item: DetailedItemProperties = DetailedItemProperties{
        id: cv.id,
        date_created: cv.date_created.to_string(),
        props: detailed_props,
    };

    render! {
        div {
            div { class: "flex flex-row items-center justify-center bg-gray-200",
                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"simple_list",
                    model: &"CV",
                    headers_vec: cv_simple_headers_vec.clone(),
                    detailed_headers_vec: cv_detailed_headers_vec.clone(),
                    item_vec: item_vec.clone(),
                    detailed_item: detailed_item.clone(),
                },

                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"detailed_view",
                    model: &"CV",
                    headers_vec: cv_simple_headers_vec,
                    detailed_headers_vec: cv_detailed_headers_vec,
                    item_vec: item_vec,
                    detailed_item: detailed_item,
                },
            },
        },
    }
}
