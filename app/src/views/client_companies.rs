#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{
    CurrentFilters,
    services::client_companies::{get_client_companies, get_client_company},
    views::{shared::Card, SimpleProps, DetailedProps, SimpleItemProperties, DetailedItemProperties}
};
use crate::CurrentDetailedObjects;

// Set here what you want to show, id and date_created are already passed to the component
const CLIENT_COMPANY_SIMPLE_HEADERS: [&str; 1] = ["company_name"];
const CLIENT_COMPANY_DETAILED_HEADERS: [&str; 2] = ["company_name", "date_created"];

#[component]
pub fn ClientCompanies(cx: Scope) -> Element {
    let title: String = "Client Companies".to_string();
    let subtitle: String = "ttt2".to_string();
    let currentFilterStruct = use_shared_state::<CurrentFilters>(cx).unwrap();

    let client_company_vec = use_future!(cx, |currentFilterStruct| async move {
        get_client_companies(
            999,
            0,
            "company_name".to_owned(),
            "ASC".to_owned(),
            currentFilterStruct.read().ClientCompany.clone(),
        )
        .await
        .unwrap()
    })
    .value()?;

    let currentDetailStruct = use_shared_state::<CurrentDetailedObjects>(cx).unwrap();

    let client_company = use_future!(cx, |currentDetailStruct| async move {
        get_client_company(currentDetailStruct.read().ClientCompany)
        .await
        .unwrap()
    })
    .value()?;

    let client_company_simple_headers_vec: Vec<&'static str> = Vec::from(CLIENT_COMPANY_SIMPLE_HEADERS);
    let client_company_detailed_headers_vec: Vec<&'static str> = Vec::from(CLIENT_COMPANY_DETAILED_HEADERS);

    let mut props_vec: Vec<SimpleProps> = vec![];
    for item in client_company_vec {
        props_vec.push(
            (&item.company_name, "", "", "", "")
        );
    }

    let item_vec: Vec<SimpleItemProperties> = client_company_vec.clone().iter().enumerate().map(|item| SimpleItemProperties{
        id: item.1.id,
        date_created: item.1.date_created.to_string(),
        props: props_vec[item.0],
    }).collect();

    let detailed_props: DetailedProps = (&client_company.company_name, "", "", "", "", "", "", "");

    let detailed_item: DetailedItemProperties = DetailedItemProperties{
        id: client_company.id,
        date_created: client_company.date_created.to_string(),
        props: detailed_props,
    };

    render! {
        div {
            div { class: "flex flex-row items-center justify-center bg-gray-200",
                rsx! {
                    form {
                        onsubmit: move |event| {
                            currentFilterStruct.write().ClientCompany = "company_name,".to_owned() + "" + &event.data.values["name"][0].clone();
                        },

                        label {
                            class: "mx-1 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                            "Name",
                            input {
                                class: "mx-5 text-gray-600 py-1 px-4 rounded",
                                name: "name",
                                },
                        },
                        input { r#type: "submit", value: "Filter Clients", class: "mx-1 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" },
                    }
                }
            },

            div { class: "flex flex-row items-center justify-center bg-gray-200",
                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"simple_list",
                    model: &"ClientCompany",
                    headers_vec: client_company_simple_headers_vec.clone(),
                    detailed_headers_vec: client_company_detailed_headers_vec.clone(),
                    item_vec: item_vec.clone(),
                    detailed_item: detailed_item.clone(),
                },

                Card {
                    card_title: title,
                    card_subtitle: subtitle,
                    r#type: &"detailed_view",
                    model: &"ClientCompany",
                    headers_vec: client_company_detailed_headers_vec.clone(),
                    detailed_headers_vec: client_company_detailed_headers_vec.clone(),
                    item_vec: item_vec.clone(),
                    detailed_item: detailed_item.clone(),
                },
            },
        },
    }
}
