use dioxus::prelude::*;
use dioxus_router::components::Outlet;

use crate::router::Route;

#[component]
pub fn Card(cx: Scope, card_title: String, card_content: Element) -> Element {
    cx.render({ rsx!{
        div { class: "flex items-center justify-center min-h-screen bg-gray-200",
            div { aria_label: "card", class: "p-8 rounded-3xl bg-white max-w-sm w-full",
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
                            "\n            Daily usage\n          "
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
                }
                div { aria_label: "content", class: "mt-9 grid gap-2.5",
                    a { href: "#",
                        div { class: "flex items-center space-x-4 p-3.5 rounded-full bg-gray-100",
                            span { class: "flex items-center justify-center w-10 h-10 shrink-0 rounded-full bg-white text-gray-900",
                                svg {
                                    width: "24",
                                    stroke_linecap: "round",
                                    stroke: "currentColor",
                                    height: "24",
                                    stroke_width: "1.5",
                                    stroke_linejoin: "round",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    class: "w-6 h-6",
                                    path { fill: "none", stroke: "none", d: "M0 0h24v24H0z" }
                                    path { d: "M8 16a3 3 0 0 1 -3 3" }
                                    path { d: "M16 16a3 3 0 0 0 3 3" }
                                    path { d: "M12 16v4" }
                                    path { d: "M3 5m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z" }
                                    path { d: "M7 13v-3a1 1 0 0 1 1 -1h8a1 1 0 0 1 1 1v3" }
                                }
                            }
                            div { class: "flex flex-col flex-1",
                                h3 { class: "text-sm font-medium", "Air Conditioner" }
                                div { class: "divide-x divide-gray-200 mt-auto",
                                    span { class: "inline-block px-3 text-xs leading-none text-gray-400 font-normal first:pl-0",
                                        "2 unit"
                                    }
                                    span { class: "inline-block px-3 text-xs leading-none text-gray-400 font-normal first:pl-0",
                                        "18kWh"
                                    }
                                }
                            }
                            svg {
                                stroke_linecap: "round",
                                xmlns: "http://www.w3.org/2000/svg",
                                stroke_linejoin: "round",
                                fill: "none",
                                stroke: "currentColor",
                                width: "24",
                                stroke_width: "2",
                                height: "24",
                                view_box: "0 0 24 24",
                                class: "w-5 h-5 shrink-0",
                                path { stroke: "none", d: "M0 0h24v24H0z", fill: "none" }
                                path { d: "M9 6l6 6l-6 6" }
                            }
                        }
                    }
                    a { href: "#",
                        div { class: "flex items-center space-x-4 p-3.5 rounded-full bg-gray-100",
                            span { class: "flex items-center justify-center w-10 h-10 shrink-0 rounded-full bg-white text-gray-900",
                                svg {
                                    height: "24",
                                    stroke_width: "1.5",
                                    stroke: "currentColor",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    stroke_linejoin: "round",
                                    width: "24",
                                    stroke_linecap: "round",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    class: "w-6 h-6",
                                    path { stroke: "none", fill: "none", d: "M0 0h24v24H0z" }
                                    path { d: "M3 13m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v4a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z" }
                                    path { d: "M17 17l0 .01" }
                                    path { d: "M13 17l0 .01" }
                                    path { d: "M15 13l0 -2" }
                                    path { d: "M11.75 8.75a4 4 0 0 1 6.5 0" }
                                    path { d: "M8.5 6.5a8 8 0 0 1 13 0" }
                                }
                            }
                            div { class: "flex flex-col flex-1",
                                h3 { class: "text-sm font-medium", "Wi-Fi Router" }
                                div { class: "divide-x divide-gray-200 mt-auto",
                                    span { class: "inline-block px-3 text-xs leading-none text-gray-400 font-normal first:pl-0",
                                        "1 unit"
                                    }
                                    span { class: "inline-block px-3 text-xs leading-none text-gray-400 font-normal first:pl-0",
                                        "8kWh"
                                    }
                                }
                            }
                            svg {
                                width: "24",
                                stroke_width: "2",
                                stroke_linejoin: "round",
                                stroke: "currentColor",
                                fill: "none",
                                height: "24",
                                view_box: "0 0 24 24",
                                stroke_linecap: "round",
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "w-5 h-5 shrink-0",
                                path { stroke: "none", d: "M0 0h24v24H0z", fill: "none" }
                                path { d: "M9 6l6 6l-6 6" }
                            }
                        }
                    }
                    a { href: "#",
                        div { class: "flex items-center space-x-4 p-3.5 rounded-full bg-gray-100",
                            span { class: "flex items-center justify-center w-10 h-10 shrink-0 rounded-full bg-white text-gray-900",
                                svg {
                                    stroke_linecap: "round",
                                    width: "24",
                                    height: "24",
                                    stroke_linejoin: "round",
                                    stroke: "currentColor",
                                    view_box: "0 0 24 24",
                                    stroke_width: "1.5",
                                    fill: "none",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    class: "w-6 h-6",
                                    path { stroke: "none", d: "M0 0h24v24H0z", fill: "none" }
                                    path { d: "M3 5a1 1 0 0 1 1 -1h16a1 1 0 0 1 1 1v10a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-10z" }
                                    path { d: "M7 20h10" }
                                    path { d: "M9 16v4" }
                                    path { d: "M15 16v4" }
                                }
                            }
                            div { class: "flex flex-col flex-1",
                                h3 { class: "text-sm font-medium", "Smart Tv" }
                                div { class: "divide-x divide-gray-200 mt-auto",
                                    span { class: "inline-block px-3 text-xs leading-none text-gray-400 font-normal first:pl-0",
                                        "2 unit"
                                    }
                                    span { class: "inline-block px-3 text-xs leading-none text-gray-400 font-normal first:pl-0",
                                        "12kWh"
                                    }
                                }
                            }
                            svg {
                                fill: "none",
                                width: "24",
                                stroke_linejoin: "round",
                                xmlns: "http://www.w3.org/2000/svg",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                height: "24",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                class: "w-5 h-5 shrink-0",
                                path { stroke: "none", d: "M0 0h24v24H0z", fill: "none" }
                                path { d: "M9 6l6 6l-6 6" }
                            }
                        }
                    }
                    a { href: "#",
                        div { class: "flex items-center space-x-4 p-3.5 rounded-full bg-gray-100",
                            span { class: "flex items-center justify-center w-10 h-10 shrink-0 rounded-full bg-white text-gray-900",
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "24",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    height: "24",
                                    stroke_width: "1.5",
                                    class: "w-6 h-6",
                                    path { stroke: "none", d: "M0 0h24v24H0z", fill: "none" }
                                    path { d: "M7.502 19.423c2.602 2.105 6.395 2.105 8.996 0c2.602 -2.105 3.262 -5.708 1.566 -8.546l-4.89 -7.26c-.42 -.625 -1.287 -.803 -1.936 -.397a1.376 1.376 0 0 0 -.41 .397l-4.893 7.26c-1.695 2.838 -1.035 6.441 1.567 8.546z" }
                                }
                            }
                            div { class: "flex flex-col flex-1",
                                h3 { class: "text-sm font-medium", "Humidifier" }
                                div { class: "divide-x divide-gray-200 mt-auto",
                                    span { class: "inline-block px-3 text-xs leading-none text-gray-400 font-normal first:pl-0",
                                        "1 unit"
                                    }
                                    span { class: "inline-block px-3 text-xs leading-none text-gray-400 font-normal first:pl-0",
                                        "2kWh"
                                    }
                                }
                            }
                            svg {
                                height: "24",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                width: "24",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                fill: "none",
                                stroke: "currentColor",
                                xmlns: "http://www.w3.org/2000/svg",
                                class: "w-5 h-5 shrink-0",
                                path { d: "M0 0h24v24H0z", stroke: "none", fill: "none" }
                                path { d: "M9 6l6 6l-6 6" }
                            }
                        }
                    }
                }
            }
        }
    }   })
}

// #[component]
// pub fn List(cx: Scope) -> Element {
//     render! { rsx!{
//     } }
// }