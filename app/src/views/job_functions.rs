#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{
    services::job_functions::{get_job_functions, get_job_function},
    views::{shared::Card, SimpleProps, DetailedProps, SimpleItemProperties, DetailedItemProperties}
};

const JOB_FUNCTION_SIMPLE_HEADERS: [&str; 1] = ["job_function_name"];
const JOB_FUNCTION_DETAILED_HEADERS: [&str; 2] = ["job_function_name", "date_created"];

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

    let job_function = use_future!(cx, || async move {
        get_job_function(1)
        .await
        .unwrap()
    })
    .value()?;

    let job_function_simple_headers_vec: Vec<&'static str> = Vec::from(JOB_FUNCTION_SIMPLE_HEADERS);
    let job_function_detailed_headers_vec: Vec<&'static str> = Vec::from(JOB_FUNCTION_DETAILED_HEADERS);

    // Set here what you want to show, should match JOB_FUNCTION_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<SimpleProps> = vec![];
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

    let detailed_props: DetailedProps = (&job_function.job_function_name, "", "", "", "", "", "", "");

    let detailed_item: DetailedItemProperties = DetailedItemProperties{
        id: job_function.id,
        date_created: job_function.date_created.to_string(),
        props: detailed_props,
    };

    render! {
        div {
            div { class: "flex flex-row items-center justify-center bg-gray-200",
                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"simple_list",
                    model: &"JobFunction",
                    headers_vec: job_function_simple_headers_vec.clone(),
                    detailed_headers_vec: job_function_detailed_headers_vec.clone(),
                    item_vec: item_vec.clone(),
                    detailed_item: detailed_item.clone(),
                },

                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"detailed_view",
                    model: &"JobFunction",
                    headers_vec: job_function_simple_headers_vec,
                    detailed_headers_vec: job_function_detailed_headers_vec.clone(),
                    item_vec: item_vec.clone(),
                    detailed_item: detailed_item.clone(),
                },
            },
        },
    }
}
