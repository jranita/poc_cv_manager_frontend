use dioxus::{prelude::*};
pub(crate) use dioxus_router::components::Link;

use crate::router::Route;

#[component]
pub fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(
        Html (
            <header>
                <script src: "../../node_modules/flowbite/dist/flowbite.min.js" />
            </header>
            <body>
                // Link {
                //     class: "text-red-400 bg-gray-300 flex items-center justify-center",
                //     to: Route::Blog {
                //         id: *count.get()
                //     },
                //     "Go to blog!!"
                // },
                // div {
                //     h1 { "High-Five counter: {count}" }
                //     button { onclick: move |_| count += 1, "Up high!" }
                //     button { onclick: move |_| count -= 1, "Down low!" }
                // }
                <div inline-datepicker data-date="02/25/2022"></div>
            </body>
        )
    )
}