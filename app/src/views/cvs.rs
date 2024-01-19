use dioxus::{html, prelude::*};
use dioxus_router::components::Link;

use crate::{models::cv::CV, router::Route, services::cvs::get_cvs};

#[component]
pub fn Cvs(cx: Scope) -> Element {
    let cv_vec = use_future!(cx, || async move {
        get_cvs(
            999,
            0,
            "cv_name".to_owned(),
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
            cv_vec.iter().map(|item| {
                    let CV {id,cv_name,date_created, user_id, file_name, keyword_list, target_companies, target_job_functions } = item;
                    rsx!(
                        li {"{cv_name}"}
                    )
                }
            )
        }

        // format!("CVs List +{:?}+", cv_vec)
    }
}
