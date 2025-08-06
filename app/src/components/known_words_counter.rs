use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn KnownWordsCounter(
    count: usize,
    theme: Theme,
    on_click: EventHandler<()>,
) -> Element {
    let display_count = if count > 999 {
        "999+".to_string()
    } else {
        count.to_string()
    };

    rsx! {
        button {
            class: "known-words-counter",
            style: "
                position: fixed;
                top: 80px;
                right: 20px;
                z-index: 100;
                background: {theme.accent};
                border: 2px solid {theme.border};
                border-radius: 50%;
                width: 50px;
                height: 50px;
                display: flex;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                transition: all 0.3s ease;
                box-shadow: 0 2px 8px {theme.shadow};
                color: white;
                font-size: 0.9em;
                font-weight: 600;
                min-width: 50px;
                min-height: 50px;
            ",
            onclick: move |_| on_click.call(()),
            title: "View known words ({count})",
            onmouseenter: |_| {},
            onmouseleave: |_| {},
            
            span {
                style: "
                    line-height: 1;
                    text-align: center;
                    overflow: hidden;
                    text-overflow: ellipsis;
                    white-space: nowrap;
                    max-width: 35px;
                ",
                "{display_count}"
            }
        }
    }
}
