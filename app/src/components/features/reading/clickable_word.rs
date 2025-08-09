use dioxus::prelude::*;

#[component]
pub fn ClickableWord(
    text: String,
    index: usize,
    is_clickable: bool,
    style: String,
    on_click: EventHandler<String>,
) -> Element {
    if is_clickable {
        rsx! {
            span {
                key: "clickable_{index}",
                style: "{style}; cursor: pointer; user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                ondoubleclick: move |_| on_click.call(text.clone()),
                "{text}"
            }
        }
    } else {
        rsx! {
            span {
                key: "non_clickable_{index}",
                style: "{style}; user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                "{text}"
            }
        }
    }
}
