use dioxus::prelude::*;
use crate::theme::Theme;

/// Displays loading state for content
#[component]
pub fn LoadingState(theme: Theme) -> Element {
    rsx! {
        div {
            class: "loading-overlay",
            style: {
                let bg_color = if theme.background.contains("#fff") || theme.background.contains("255") {
                    "rgba(255, 255, 255, 0.4)"
                } else {
                    "rgba(0, 0, 0, 0.1)"
                };
                format!(
                    "position: absolute; top: 0; left: 0; right: 0; bottom: 0; \
                     display: flex; align-items: center; justify-content: center; \
                     background: {}; z-index: 5;",
                    bg_color
                )
            },
            
            div {
                class: "loading-spinner",
                style: "
                    width: 24px;
                    height: 24px;
                    border: 2px solid {theme.border};
                    border-top-color: {theme.accent};
                    border-radius: 50%;
                    animation: spin 0.8s ease-in-out infinite;
                    opacity: 0.7;
                ",
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