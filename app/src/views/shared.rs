#![allow(non_snake_case)]
use chrono::DateTime;
use dioxus::prelude::*;
 use crate::dioxus_elements::span;
use super::SimpleItemProperties;

#[component]
pub fn Card<'a>(
    cx: Scope,
    card_title: String,
    r#type: &'a str,
    card_subtitle: String,
    model: &'a str,
    headers_vec: Vec<&'static str>,
    item_vec: Vec<SimpleItemProperties<'a>>,
) -> Element {
    cx.render({ rsx!{
        div { class: "flex items-center justify-center bg-gray-200",
            div { aria_label: "card", class: "p-8 m-10 rounded-3xl bg-white max-w-sm w-full",
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
                div {
                    aria_label: "content",
                    style: "height: 40svh",
                    class: "mt-9 grid gap-2.5 overflow-scroll {model}",

                    for item in item_vec {
                        rsx!(
                            div {
                                key: "{item.id}",
                                id: "{item.id}",
                                for (i, header) in headers_vec.iter().enumerate() {
                                    match i {
                                        // 0 => format!("{}: {}", (*header).to_string(), item.props.0),
                                        0 => format!("{} ", item.props.0),
                                        1 => format!(" {}", item.props.1),
                                        2 => format!(" {}", item.props.2),
                                        3 => format!(" {}", item.props.3),
                                        4 => format!(" {}", item.props.4),
                                        _ => "".to_string()
                                    }
                                }
                            }
                        )
                    }
                }
            }
        }
    }})
}
