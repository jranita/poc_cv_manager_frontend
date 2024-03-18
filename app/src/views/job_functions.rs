#![allow(non_snake_case)]
use crate::CurrentDetailedObjects;
use crate::{
    CurrentFilters,
    services::job_functions::{get_job_function, get_job_functions},
    views::{
        shared::Card, DetailedItemProperties, DetailedProps, SimpleItemProperties, SimpleProps,
    },
};
use dioxus::prelude::*;

const JOB_FUNCTION_SIMPLE_HEADERS: [&str; 1] = ["job_function_name"];
const JOB_FUNCTION_DETAILED_HEADERS: [&str; 2] = ["job_function_name", "date_created"];

#[component]
pub fn JobFunctions(cx: Scope) -> Element {
    let title: String = "Job Titles".to_string();
    let subtitle: String = "ttt3".to_string();
    let currentFilterStruct = use_shared_state::<CurrentFilters>(cx).unwrap();

    let job_function_vec = use_future!(cx, |currentFilterStruct| async move {
         get_job_functions(
            999,
            0,
            "job_function_name".to_owned(),
            "ASC".to_owned(),
            currentFilterStruct.read().CV.clone(),
        )
        .await
        .unwrap()
    })
    .value()?;

    let currentDetailStruct = use_shared_state::<CurrentDetailedObjects>(cx).unwrap();

    let job_function = use_future!(cx, |currentDetailStruct| async move {
        get_job_function(currentDetailStruct.read().JobFunction)
            .await
            .unwrap()
    })
    .value()?;

    let job_function_simple_headers_vec: Vec<&'static str> = Vec::from(JOB_FUNCTION_SIMPLE_HEADERS);
    let job_function_detailed_headers_vec: Vec<&'static str> =
        Vec::from(JOB_FUNCTION_DETAILED_HEADERS);

    // Set here what you want to show, should match JOB_FUNCTION_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<SimpleProps> = vec![];
    for item in job_function_vec {
        props_vec.push((&item.job_function_name, "", "", "", ""));
    }

    let item_vec: Vec<SimpleItemProperties> = job_function_vec
        .clone()
        .iter()
        .enumerate()
        .map(|item| SimpleItemProperties {
            id: item.1.id,
            date_created: item.1.date_created.to_string(),
            props: props_vec[item.0],
        })
        .collect();

    let detailed_props: DetailedProps =
        (&job_function.job_function_name, "", "", "", "", "", "", "");

    let detailed_item: DetailedItemProperties = DetailedItemProperties {
        id: job_function.id,
        date_created: job_function.date_created.to_string(),
        props: detailed_props,
    };

    render! {
        div {
            div { class: "flex flex-row items-center justify-center bg-gray-200",
                rsx! {
                    form {
                        onsubmit: move |event| {
                            currentFilterStruct.write().CV = "job_function_name,".to_owned() + "" + &event.data.values["filter"][0].clone();
                        },

                        label {
                            class: "mx-1 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                            "Name",
                            input {
                                class: "mx-5 text-gray-600 py-1 px-4 rounded",
                                name: "filter",
                            }
                        },

                        label {
                            class: "mx-1 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                            "Keywords",
                            select {
                                class: "mx-5 text-gray-600 py-1 px-4 rounded",
                                option {
                                    value: "test",
                                    "test1"
                                },
                                option {
                                    value: "test",
                                    "test2"
                                },

                            },
                        },

                        input { r#type: "submit", value: "Filter Job Functions", class: "mx-1 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" },
                    }
                }
            },

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
