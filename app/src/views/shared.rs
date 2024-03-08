#![allow(non_snake_case)]
use crate::CurrentDetailedObjects;

use super::{DetailedItemProperties, SimpleItemProperties};
use dioxus::prelude::*;

pub struct ComplexData(usize);

// #[derive(Props)]
// pub struct ListButtonProps<'a> {
//     on_click: EventHandler<'a, ComplexData>,
// }

// pub fn ListButton<'a>(cx: Scope<'a, ListButtonProps<'a>>) -> Element<'a> {
//     cx.render(rsx!(button {
//         class: "fancy-button",
//         onclick: move |_| cx.props.on_click.call(ComplexData(0)),
//         "click me pls."
//     }))
// }

#[component]
pub fn Card<'a>(
    cx: Scope,
    card_title: String,
    r#type: &'a str,
    card_subtitle: String,
    model: &'a str,
    headers_vec: Vec<&'static str>,
    detailed_headers_vec: Vec<&'static str>,
    item_vec: Vec<SimpleItemProperties<'a>>,
    detailed_item: DetailedItemProperties<'a>,
) -> Element {
    cx.render({
        log::info!("cx.render Card {}", r#type);
        rsx!{
            div { aria_label: "card", class: "p-8 m-10 rounded-3xl bg-white max-w-sm w-full",
                if r#type == &"simple_list"  {
                    cx.render(
                        {rsx!{
                            div { aria_label: "header", class: "flex items-center space-x-2",
                                svg {
                                    width: "24",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    fill: "none",
                                    height: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    stroke_width: "1.5",
                                    class: "w-8 h-8 shrink-0",
                                    path { stroke: "none", fill: "none", d: "M0 0h24v24H0z" }
                                    path { d: "M13 3l0 7l6 0l-8 11l0 -7l-6 0l8 -11" }
                                }
                                div { class: "space-y-0.5 flex-1",
                                    h3 { class: "font-medium text-lg tracking-tight text-gray-900 leading-tight",
                                        "\n {card_title} \n          "
                                    }
                                    p { class: "text-sm font-normal text-gray-400 leading-none",
                                        "\n            {card_subtitle}\n          "
                                    }
                                }
                                a {
                                    href: "/",
                                    class: "inline-flex items-center shrink-0 justify-center w-8 h-8 rounded-full text-white bg-gray-900 focus:outline-none",
                                    svg {
                                        view_box: "0 0 24 24",
                                        stroke_width: "1.5",
                                        height: "24",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke: "currentColor",
                                        fill: "none",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        class: "w-5 h-5",
                                        path { stroke: "none", d: "M0 0h24v24H0z", fill: "none" }
                                        path { d: "M17 7l-10 10" }
                                        path { d: "M8 7l9 0l0 9" }
                                    }
                                }
                                span {}
                            }
                            SimpleList {
                                model: model,
                                headers_vec: headers_vec.to_vec(),
                                item_vec: item_vec.to_vec(),
                            }

                        }}
                    )
                }

                if r#type == &"detailed_view"  {
                    cx.render(
                        {rsx!{
                            div { aria_label: "header", class: "flex items-center space-x-2",
                                svg {
                                    width: "24",
                                    view_box: "0 0 24 24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    fill: "none",
                                    height: "24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    stroke_width: "1.5",
                                    class: "w-8 h-8 shrink-0",
                                    path { stroke: "none", fill: "none", d: "M0 0h24v24H0z" }
                                    path { d: "M13 3l0 7l6 0l-8 11l0 -7l-6 0l8 -11" }
                                }
                                div { class: "space-y-0.5 flex-1",
                                    h3 { class: "font-medium text-lg tracking-tight text-gray-900 leading-tight",
                                        "\n {card_title} \n          "
                                    }
                                    p { class: "text-sm font-normal text-gray-400 leading-none",
                                        "\n            {card_subtitle}\n          "
                                    }
                                }
                                a {
                                    href: "/",
                                    class: "inline-flex items-center shrink-0 justify-center w-8 h-8 rounded-full text-white bg-gray-900 focus:outline-none",
                                    svg {
                                        view_box: "0 0 24 24",
                                        stroke_width: "1.5",
                                        height: "24",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        stroke: "currentColor",
                                        fill: "none",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "24",
                                        class: "w-5 h-5",
                                        path { stroke: "none", d: "M0 0h24v24H0z", fill: "none" }
                                        path { d: "M17 7l-10 10" }
                                        path { d: "M8 7l9 0l0 9" }
                                    }
                                }
                                span {}
                            }

                            DetailedView {
                                model: model,
                                detailed_headers_vec: detailed_headers_vec.to_vec(),
                                detailed_item: detailed_item,
                            }
                        }}
                    )
                }
            }
        }
    })
}

#[component]
pub fn SimpleList<'a>(
    cx: Scope<'a, CustomFancyButtonProps<'a>>,
    model: &'a str,
    headers_vec: Vec<&'static str>,
    item_vec: Vec<SimpleItemProperties<'a>>,
) -> Element<'a> {
    let currentDetailStruct = use_shared_state::<CurrentDetailedObjects>(cx).unwrap();
    cx.render({
        rsx! {
            div {
                aria_label: "content",
                style: "height: 40svh",
                class: "mt-9 grid gap-2.5 overflow-scroll {model} simple-list",

                for item in item_vec {
                    rsx!(
                        div {
                            onclick: move |_| {
                                if *model == "Keyword" {
                                    currentDetailStruct.write().Keyword = item.id;
                                    log::info!("Clicked178! id: {:?}", currentDetailStruct.read().Keyword);
                                } else if *model == "ClientCompany" {
                                    currentDetailStruct.write().ClientCompany = item.id;
                                    log::info!("Clicked181! id: {:?}", currentDetailStruct.read().ClientCompany);
                                } else if *model == "CV" {
                                    currentDetailStruct.write().CV = item.id;
                                    log::info!("Clicked181! id: {:?}", currentDetailStruct.read().CV);
                                } else if *model == "JobFunction" {
                                    currentDetailStruct.write().JobFunction = item.id;
                                    log::info!("Clicked181! id: {:?}", currentDetailStruct.read().JobFunction);
                                } else if *model == "User" {
                                    currentDetailStruct.write().User = item.id;
                                    log::info!("Clicked181! id: {:?}", currentDetailStruct.read().User);
                                }
                            },
                            key: "{item.id}",
                            id: "{item.id}",
                            for (i, _header) in headers_vec.iter().enumerate() {
                                match i {
                                    // 0 => format!("{}: {}", (*header).to_string(), item.props.0),
                                    0 => format!("{} ", item.props.0),
                                    1 => format!(" {}", item.props.1),
                                    2 => format!(" {}", item.props.2),
                                    3 => format!(" {}", item.props.3),
                                    4 => format!(" {}", item.props.4),
                                    _ => "".to_string()
                                }
                            },
                        }
                    )
                }
            }
        }
    })
}

#[component]
pub fn DetailedView<'a>(
    cx: Scope,
    model: &'a str,
    detailed_headers_vec: Vec<&'static str>,
    detailed_item: &'a DetailedItemProperties<'a>,
) -> Element<'a> {
    let currentDetailStruct = use_shared_state::<CurrentDetailedObjects>(cx).unwrap();
    cx.render({
        rsx! {
            div {
                aria_label: "content",
                style: "height: 40svh",
                class: "mt-9 grid gap-2.5 overflow-scroll {model} detailed-view",

                rsx!(
                    div {
                        key: "{detailed_item.id}",
                        // id: "{detailed_item.id}",
                        id: "{detailed_item.id}",
                        for (i, _header) in detailed_headers_vec.iter().enumerate() {
                            match i {
                                0 => format!("{} ", detailed_item.props.0),
                                1 => format!("{} ", detailed_item.props.1),
                                2 => format!("{} ", detailed_item.props.2),
                                3 => format!("{} ", detailed_item.props.3),
                                4 => format!("{} ", detailed_item.props.4),
                                5 => format!("{} ", detailed_item.props.5),
                                6 => format!("{} ", detailed_item.props.6),
                                7 => format!("{} ", detailed_item.props.7),
                                _ => "".to_string()
                            }
                        }
                    }
                )
            }
        }
    })
}
