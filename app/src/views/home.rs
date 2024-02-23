use dioxus::prelude::*;

#[component]
pub fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    fn change_count(id: i32, mut count: &UseState<i32>) {
        count += id;
    }

    cx.render(rsx! (
        body {
          class: "bg-gray-100",
          div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| change_count(10, count), "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
          }
        }
    ))
}
