use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn ProgressBar(current: usize, total: usize, theme: Theme) -> Element {
    let percentage = if total > 0 { (current as f32 / total as f32) * 100.0 } else { 0.0 };

    rsx! {
        div {
            class: "progress-bar-container",
            style: "position: sticky; top: 0; width: 100%; height: 3px; background: {theme.border}; z-index: 50;",
            div {
                class: "progress-bar-filler",
                style: "width: {percentage}%; height: 100%; background: {theme.accent};",
            }
        }
    }
}
