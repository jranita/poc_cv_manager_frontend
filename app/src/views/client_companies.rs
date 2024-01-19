use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{
    models::client_company::ClientCompany, router::Route,
    services::client_companies::get_client_companies,
};

#[component]
pub fn ClientCompanies(cx: Scope) -> Element {
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
    render! {
        Link { to: Route::Home {}, "Home" },
        div {
            client_company_vec.iter().map(|item| {
                    let ClientCompany {id, company_name, date_created} = item;
                    rsx!(
                        li {"{company_name}"}
                    )
                }
            )
        }

        // format!("Keywords List +{:?}+", keyword_vec)
    }
}
