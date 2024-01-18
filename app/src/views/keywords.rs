use dioxus::{html, prelude::*};
use dioxus_router::components::Link;

use crate::{models::keyword::Keyword, router::Route, services::keywords::get_keywords};

#[component]
pub fn Keywords(cx: Scope) -> Element {
    let keyword_vec = use_future!(cx, || async move {
        get_keywords(
            999,
            0,
            "keyword_name".to_owned(),
            "ASC".to_owned(),
            "keyword_name,ac".to_owned(),
        )
        .await
        .unwrap()
    })
    .value()?;
    render! {
        Link { to: Route::Home {}, "Home" },
        div {
            keyword_vec.iter().map(|item| {
                    let Keyword {id, keyword_name, date_created} = item;
                    rsx!(
                        li {"{keyword_name}"}
                    )
                }
            )
        }

        // format!("Keywords List +{:?}+", keyword_vec)
    }
}
