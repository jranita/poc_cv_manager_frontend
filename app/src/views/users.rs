use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{models::user::User, router::Route, services::users::get_users};

#[component]
pub fn Users(cx: Scope) -> Element {
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
    render! {
        Link { to: Route::Home {}, "Home" },
        div {
            user_vec.iter().map(|item| {
                    let User { id, firstname, lastname, email, pass, role, cv_id_list, date_created } = item;
                    rsx!(
                        li {"{firstname} {lastname}"}
                    )
                }
            )
        }

        // format!("Keywords List +{:?}+", keyword_vec)
    }
}
