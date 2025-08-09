use dioxus::prelude::*;
use crate::theme::Theme;

/// Displays loading state for content
#[component]
pub fn LoadingState(theme: Theme) -> Element {
    rsx! {
        div {
            class: "loading-container",
            style: "display: flex; align-items: center; justify-content: center; padding: 40px;",
            
            div {
                class: "loading-spinner",
                style: "width: 40px; height: 40px; border: 3px solid {theme.border}; border-top-color: {theme.accent}; border-radius: 50%; animation: spin 1s linear infinite;",
            }
            
            style {
                "@keyframes spin {{
                    from {{ transform: rotate(0deg); }}
                    to {{ transform: rotate(360deg); }}
                }}"
            }
        }
    }
}