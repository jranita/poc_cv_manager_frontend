use dioxus::prelude::*;
pub(crate) use dioxus_router::components::Link;

use crate::router::Route;

#[component]
pub fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(
        rsx! (
            body {
                Link {
                    class: "text-red-400 bg-gray-300 flex items-center justify-center",
                    to: Route::Keywords {  } {
                        // id: *count.get()
                    },
                    "Go to keywords!!"
                },
                div {
                    h1 { "High-Five counter: {count}" }
                    button { onclick: move |_| count += 1, "Up high!" }
                    button { onclick: move |_| count -= 1, "Down low!" }

                }
            }
        )
    )
}