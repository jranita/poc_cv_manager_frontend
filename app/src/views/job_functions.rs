#![allow(non_snake_case)]
use chrono::DateTime;
use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{
    models::job_function::JobFunction, router::Route, services::job_functions::get_job_functions, views::{shared::Card, Props, SimpleItemProperties}};


const JOB_FUNCTION_SIMPLE_HEADERS: [&str; 1] = ["company_name"];

#[component]
pub fn JobFunctions(cx: Scope) -> Element {
    let title: String = "Job Titles".to_string();
    let subtitle: String = "ttt3".to_string();
    let job_function_vec = use_future!(cx, || async move {
        get_job_functions(
            999,
            0,
            "job_function_name".to_owned(),
            "ASC".to_owned(),
            "".to_owned(),
        )
        .await
        .unwrap()
    })
    .value()?;

    let job_function_simple_headers_vec: Vec<&'static str> = Vec::from(JOB_FUNCTION_SIMPLE_HEADERS);

    // Set here what you want to show, should match JOB_FUNCTION_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<Props> = vec![];
    for item in job_function_vec {
        props_vec.push(
            (&item.job_function_name, "", "", "", "")
        );
    }

    let item_vec: Vec<SimpleItemProperties> = job_function_vec.clone().iter().enumerate().map(|item| SimpleItemProperties{
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
                model: "JobFunction".to_string(),
                headers_vec: job_function_simple_headers_vec,
                item_vec: item_vec,
            },
        },

        div {
            Link { to: Route::Home {}, "Home2" },
        }
    }
}
