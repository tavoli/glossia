use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn NavigationControls(
    theme: Theme,
    on_prev: EventHandler<()>,
    on_next: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "navigation-controls",
            style: "margin-top: 30px; display: flex; gap: 10px; justify-content: center;",
            
            button { 
                style: "background: {theme.accent}; color: white; border: none; padding: 10px 20px; border-radius: 6px; cursor: pointer; font-size: 1em; transition: opacity 0.2s ease;",
                onclick: move |_| on_prev.call(()), 
                "Previous" 
            }
            
            button { 
                style: "background: {theme.accent}; color: white; border: none; padding: 10px 20px; border-radius: 6px; cursor: pointer; font-size: 1em; transition: opacity 0.2s ease;",
                onclick: move |_| on_next.call(()), 
                "Next" 
            }
        }
    }
}
