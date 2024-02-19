use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{models::user::User, router::Route, services::users::get_users, views::{shared::Card, Props, SimpleItemProperties}};

// Set here what you want to show, id and date_created are already passed to the component
const USER_SIMPLE_HEADERS: [&str; 2] = ["firstname", "lastname"];

#[component]
pub fn Users(cx: Scope) -> Element {
    let title: String = "Users".to_string();
    let subtitle: String = "ttt2".to_string();
    let user_vec = use_future!(cx, || async move {
        get_users(
            999,
            0,
            "lastname".to_owned(),
            "ASC".to_owned(),
            "".to_owned(),
        )
        .await
        .unwrap()
    })
    .value()?;

    // to sort the firstname I have to clone because original vec is not mutable
    let mut users_vec = user_vec.clone();
    // users_vec.sort_by_key(|x| x.firstname.clone());

    let user_simple_headers_vec: Vec<&'static str> = Vec::from(USER_SIMPLE_HEADERS);

    // Set here what you want to show, should match CLIENT_COMPANY_SIMPLE_HEADERS
    // TODO automate this
    let mut props_vec: Vec<Props> = vec![];
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

    let mut item_vec: Vec<SimpleItemProperties> = item_unsorted_vec.clone();
    item_vec.sort_by_key(|x| x.props.0);

    render! {
        div {
            Link { to: Route::Home {}, "Home" },

            Card {
                card_title: title.clone(),
                card_subtitle: subtitle.clone(),
                r#type: &"simple_list",
                model: &"Users",
                headers_vec: user_simple_headers_vec,
                item_vec: item_vec,
            },
        },

        div {
            Link { to: Route::Home {}, "Home2" },
        }
    }
}
