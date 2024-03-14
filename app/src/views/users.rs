#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{
    CurrentFilters,
    services::users::{get_users, get_user},
    views::{shared::Card, SimpleProps, DetailedProps, SimpleItemProperties, DetailedItemProperties}
};
use crate::CurrentDetailedObjects;

// Set here what you want to show, id and date_created are already passed to the component
const USER_SIMPLE_HEADERS: [&str; 2] = ["firstname", "lastname"];
const USER_DETAILED_HEADERS: [&str; 3] = ["firstname", "lastname", "email"];

#[component]
pub fn Users(cx: Scope) -> Element {
    let title: String = "Users".to_string();
    let subtitle: String = "ttt2".to_string();
    let currentFilterStruct = use_shared_state::<CurrentFilters>(cx).unwrap();

    let user_vec = use_future!(cx, |currentFilterStruct| async move {
        get_users(
            999,
            0,
            "lastname".to_owned(),
            "ASC".to_owned(),
            currentFilterStruct.read().CV.clone(),
        )
        .await
        .unwrap()
    })
    .value()?;

    let currentDetailStruct = use_shared_state::<CurrentDetailedObjects>(cx).unwrap();

    let user = use_future!(cx, |currentDetailStruct| async move {
        get_user(currentDetailStruct.read().User)
        .await
        .unwrap()
    })
    .value()?;

    // to sort the firstname I have to clone because original vec is not mutable
    let users_vec = user_vec.clone();
    // users_vec.sort_by_key(|x| x.firstname.clone());

    let user_simple_headers_vec: Vec<&'static str> = Vec::from(USER_SIMPLE_HEADERS);
    let user_detailed_headers_vec: Vec<&'static str> = Vec::from(USER_DETAILED_HEADERS);

    // Set here what you want to show, should match CLIENT_COMPANY_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<SimpleProps> = vec![];
    for item in user_vec {
        props_vec.push(
            (&item.firstname, &item.lastname, "", "", "")
        );
    }

    let item_unsorted_vec: Vec<SimpleItemProperties> = users_vec.clone().iter().enumerate().map(|item| SimpleItemProperties{
        id: item.1.id,
        date_created: item.1.date_created.to_string(),
        props: props_vec[item.0],
    }).collect();

    let detailed_props: DetailedProps = (&user.firstname, &user.lastname, &user.email, "", "", "", "", "");

    let detailed_item: DetailedItemProperties = DetailedItemProperties{
        id: user.id,
        date_created: user.date_created.to_string(),
        props: detailed_props,
    };

    let mut item_vec: Vec<SimpleItemProperties> = item_unsorted_vec;
    item_vec.sort_by_key(|x| x.props.0);

    render! {
        div {
            div { class: "flex flex-row items-center justify-center bg-gray-200",
                rsx! {
                    form {
                        onsubmit: move |event| {
                            currentFilterStruct.write().CV =
                                "firstname,".to_owned() + "" + &event.data.values["firstname"][0].clone() +
                                ",lastname,"+ "" + &event.data.values["lastname"][0].clone();
                        },

                        label {
                            class: "mx-1 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                            "First Name",
                            input {
                                class: "mx-5 text-gray-600 py-1 px-4 rounded",
                                name: "firstname",
                            },
                        },
                        label {
                            class: "mx-1 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                            "Last Name",
                            input {
                                class: "mx-5 text-gray-600 py-1 px-4 rounded",
                                name: "lastname",
                                },
                        },
                        input { r#type: "submit", value: "Filter Users", class: "mx-1 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" },
                    }
                }
            },

            div { class: "flex flex-row items-center justify-center bg-gray-200",
                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"simple_list",
                    model: &"User",
                    headers_vec: user_simple_headers_vec.clone(),
                    detailed_headers_vec: user_detailed_headers_vec.clone(),
                    item_vec: item_vec.clone(),
                    detailed_item: detailed_item.clone(),
                },

                Card {
                    card_title: title.clone(),
                    card_subtitle: subtitle.clone(),
                    r#type: &"detailed_view",
                    model: &"User",
                    headers_vec: user_simple_headers_vec,
                    detailed_headers_vec: user_detailed_headers_vec,
                    item_vec: item_vec,
                    detailed_item: detailed_item,
                },
            },
        },
    }
}
