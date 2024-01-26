use dioxus::{html, prelude::*};
use dioxus_router::components::Link;

use crate::{
    models::keyword::Keyword, router::Route, services::keywords::get_keywords, views::shared::Card,
};

#[component]
pub fn Keywords(cx: Scope) -> Element {
    let title: String = "Keywords".to_string();
    let subtitle: String = "ttt1".to_string();
    let keyword_vec = use_future!(cx, || async move {
        get_keywords(
            999,
            0,
            "keyword_name".to_owned(),
            "ASC".to_owned(),
            "".to_owned(),
        )
        .await
        .unwrap()
    })
    .value()?;
    render! {
        div {

            Link { to: Route::Home {}, "Home" },

            Card {
                card_title: title,
                card_subtitle: subtitle,
                item_vec: keyword_vec.clone(),
            },
        }
    }
}
