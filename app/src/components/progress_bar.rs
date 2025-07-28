use dioxus::prelude::*;

#[component]
pub fn ProgressBar(current: usize, total: usize) -> Element {
    let percentage = if total > 0 { (current as f32 / total as f32) * 100.0 } else { 0.0 };

    rsx! {
        div {
            class: "progress-bar-container",
            style: "position: sticky; top: 0; width: 100%; height: 3px; background: rgba(0,0,0,0.1); z-index: 50;",
            div {
                class: "progress-bar-filler",
                style: "width: {percentage}%; height: 100%; background: #4a90e2;",
            }
        }
    }
}
