use dioxus::prelude::*;
pub(crate) use dioxus_router::components::Link;

use crate::router::Route;

#[component]
pub fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(
        rsx! {
            Link {
                class: "w-full h-screen bg-gray-300 flex items-center justify-center",
                to: Route::Blog {
                    id: *count.get()
                },
                "Go to blog"
            },
            div {
                h1 { "High-Five counter: {count}" }
                button { onclick: move |_| count += 1, "Up high!" }
                button { onclick: move |_| count -= 1, "Down low!" }

            }
        }
    )
}