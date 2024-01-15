#![allow(non_snake_case)]

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

pub(crate) mod views;
pub(crate) mod router;
use router::Route;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");

    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        link { rel: "stylesheet", href: "/output.css" },
        Router::<Route> {}
    }
}
