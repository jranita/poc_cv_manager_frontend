#![allow(non_snake_case)]

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

pub mod views;
pub mod router;
pub mod models;
pub mod services;
use router::Route;

#[derive(Debug, PartialEq, Clone)]
struct CurrentDetailedObjects {
    ClientCompany: usize,
    CV: usize,
    JobFunction: usize,
    Keyword: usize,
    User: usize,
}

impl Default for CurrentDetailedObjects {
    fn default() -> Self {
        Self { ClientCompany: 1, CV: 1, JobFunction: 1, Keyword: 1, User: 1 }
    }
}

struct CurrentFilters {
    ClientCompany: String,
    CV: String,
    JobFunction: String,
    Keyword: String,
    User: String,
}

impl Default for CurrentFilters {
    fn default() -> Self {
        Self { ClientCompany: "".to_string(), CV: "".to_string(), JobFunction: "".to_string(), Keyword: "".to_string(), User: "".to_string() }
    }
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || CurrentDetailedObjects::default());
    use_shared_state_provider(cx, || CurrentFilters::default());

    log::info!("fn app Event:???");
    render! {
        link { rel: "stylesheet", href: "/output.css" },
        Router::<Route> {}
    }
}
