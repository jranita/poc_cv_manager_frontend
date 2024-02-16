#![allow(non_snake_case)]
use chrono::DateTime;
use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{models::cv::CV, router::Route, services::cvs::get_cvs, views::{shared::Card, Props, SimpleItemProperties}};

// Set here what you want to show, id and date_created are already passed to the component
const CV_SIMPLE_HEADERS: [&str; 1] = ["cv_name"];

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

    let cv_simple_headers_vec: Vec<&'static str> = Vec::from(CV_SIMPLE_HEADERS);

    // Set here what you want to show, should match CLIENT_COMPANY_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<Props> = vec![];
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

    render! {
        div {
            Link { to: Route::Home {}, "Home" },

            Card {
                card_title: title.clone(),
                card_subtitle: subtitle.clone(),
                r#type: "simple_list".to_string(),
                model: "CV".to_string(),
                headers_vec: cv_simple_headers_vec,
                item_vec: item_vec,
            },
        },

        div {
            Link { to: Route::Home {}, "Home2" },
        }
    }
}
