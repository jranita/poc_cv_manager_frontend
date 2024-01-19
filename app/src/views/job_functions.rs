use dioxus::{html, prelude::*};
use dioxus_router::components::Link;

use crate::{
    models::job_function::JobFunction, router::Route, services::job_functions::get_job_functions,
};

#[component]
pub fn JobFunctions(cx: Scope) -> Element {
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
    render! {
        Link { to: Route::Home {}, "Home" },
        div {
            job_function_vec.iter().map(|item| {
                    let JobFunction {id, job_function_name, date_created, keyword_list } = item;
                    rsx!(
                        li {"{job_function_name}"}
                    )
                }
            )
        }

        // format!("Keywords List +{:?}+", keyword_vec)
    }
}
