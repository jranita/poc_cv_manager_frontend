use dioxus::prelude::*;
use dioxus_router::components::Outlet;

use crate::router::Route;

#[component]
pub fn NavBar(cx: Scope) -> Element {
    render! { rsx!{
        nav {
            class: "shadow, bg-gray-100",
            div {
                class: "flex justify-between items-center py-6 px-10 container mx-auto",
                div {
                    h1 { class: "text-2xl font-bold bg-gradient-to-tr from-red-600 to-blue-600 bg-clip-text text-transparent hover:cursor-pointer",
                        p {"Prof oc Concept"},
                        p {"CV Manager"}
                    }
                }
                div {
                    div { class: "hover:cursor-pointer sm:hidden",
                        span { class: "h-1 rounded-full block w-8 mb-1 bg-gradient-to-tr from-indigo-600 to-green-600" }
                        span { class: "h-1 rounded-full block w-8 mb-1 bg-gradient-to-tr from-indigo-600 to-green-600" }
                        span { class: "h-1 rounded-full block w-8 mb-1 bg-gradient-to-tr from-indigo-600 to-green-600" }
                    }
                    div { class: "flex items-center",
                        ul { class: "sm:flex space-x-4 hidden items-center",
                            li { a {
                                href: "/",
                                class: "text-gray-700 hover:text-indigo-600 text-md",
                                "Home"
                            } }
                            li { a {
                                href: "/cvs",
                                class: "text-gray-700 hover:text-indigo-600 text-md",
                                "CVs"
                            } }
                            li { a {
                                href: "/users",
                                class: "text-gray-700 hover:text-indigo-600 text-md",
                                "Users"
                            } }
                            li { a {
                                href: "/clients",
                                class: "text-gray-700 hover:text-indigo-600 text-md",
                                "Clients"
                            } }
                            li { a {
                                href: "/job_functions",
                                class: "text-gray-700 hover:text-indigo-600 text-md",
                                "Job Functions"
                            } }
                            li { a {
                                href: "/keywords",
                                class: "text-gray-700 hover:text-indigo-600 text-md",
                                "Keywords"
                            } }
                        }
                        div { class: "md:flex items-center hidden space-x-4 ml-8 lg:ml-12",
                            h1 { class: "text-text-gray-600 py-2 hover:cursor-pointer hover:text-indigo-600",
                                "LOGIN"
                            }
                            h1 { class: "text-text-gray-600 py-2 hover:cursor-pointer px-4 rounded text-white bg-gradient-to-tr from-indigo-600 to-green-600 hover:shadow-lg",
                                "SIGNUP"
                            }
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    } }
}