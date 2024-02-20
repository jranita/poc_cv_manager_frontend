#![allow(non_snake_case)]
use chrono::DateTime;
use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{models::client_company::ClientCompany, router::Route, services::client_companies::get_client_companies, views::{shared::Card, Props, SimpleItemProperties}};

// Set here what you want to show, id and date_created are already passed to the component
const CLIENT_COMPANY_SIMPLE_HEADERS: [&str; 1] = ["company_name"];

#[component]
pub fn ClientCompanies(cx: Scope) -> Element {
    let title: String = "Client Companies".to_string();
    let subtitle: String = "ttt2".to_string();
    let client_company_vec = use_future!(cx, || async move {
        get_client_companies(
            999,
            0,
            "company_name".to_owned(),
            "ASC".to_owned(),
            "".to_owned(),
        )
        .await
        .unwrap()
    })
    .value()?;

    let client_company_simple_headers_vec: Vec<&'static str> = Vec::from(CLIENT_COMPANY_SIMPLE_HEADERS);

    // Set here what you want to show, should match CLIENT_COMPANY_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<Props> = vec![];
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

    render! {
        div {
            div { class: "flex flex-row items-center justify-center bg-gray-200",
                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"simple_list",
                    model: &"ClientCompany",
                    headers_vec: client_company_simple_headers_vec.clone(),
                    item_vec: item_vec.clone(),
                },

                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"detailed_view",
                    model: &"ClientCompany",
                    headers_vec: client_company_simple_headers_vec,
                    item_vec: item_vec,
                },
            },
        },
    }
}
