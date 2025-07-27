use dioxus::prelude::*;

#[component]
pub fn FloatingButton(count: usize, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "floating-button",
            style: "position: fixed; bottom: 20px; left: 20px; background: #4a90e2; color: white; border-radius: 50%; width: 60px; height: 60px; border: none; box-shadow: 0 2px 8px rgba(0,0,0,0.2); cursor: pointer;",
            onclick: move |_| onclick.call(()),
            "{count}"
        }
    }
}
