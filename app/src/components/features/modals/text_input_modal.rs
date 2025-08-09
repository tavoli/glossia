use dioxus::prelude::*;
use crate::components::common::modals::Modal;
use crate::theme::{use_theme, Theme};

#[component]
pub fn TextInputModal(onsubmit: EventHandler<String>, onclose: EventHandler<()>) -> Element {
    let mut text_content = use_signal(String::new);
    let theme_mode = use_theme();
    let theme = Theme::from_mode(*theme_mode.read());
    
    rsx! {
        Modal {
            theme: theme.clone(),
            on_close: Some(onclose.clone()),
            max_width: Some("600px".to_string()),
            
            div {
                class: "text-input-modal",
                style: "padding: 20px;",
                
                h2 {
                    style: "margin: 0 0 20px 0; color: {theme.text_primary}; font-size: 1.5em;",
                    "Add Text to Read"
                }
                
                textarea {
                    class: "text-input",
                    style: "
                        min-height: 200px; 
                        border: 1px solid {theme.border}; 
                        border-radius: 6px; 
                        margin-bottom: 20px; 
                        width: 100%; 
                        resize: vertical; 
                        font-family: inherit;
                        padding: 12px;
                        background: {theme.background};
                        color: {theme.text_primary};
                        box-sizing: border-box;
                    ",
                    placeholder: "Paste your text here...",
                    oninput: move |e| text_content.set(e.value()),
                    value: "{text_content}",
                    onkeydown: move |e| {
                        // Prevent Escape from bubbling when in textarea
                        if e.key() == Key::Escape {
                            e.stop_propagation();
                        }
                    }
                }
                
                div {
                    style: "display: flex; justify-content: flex-end; gap: 10px;",
                    
                    button {
                        onclick: move |_| onclose.call(()),
                        style: "
                            background: {theme.surface}; 
                            color: {theme.text_primary}; 
                            padding: 10px 20px; 
                            border: 1px solid {theme.border}; 
                            border-radius: 6px; 
                            cursor: pointer;
                            font-size: 1em;
                        ",
                        "Cancel"
                    }
                    
                    button {
                        onclick: move |_| {
                            if !text_content.read().is_empty() {
                                onsubmit.call(text_content())
                            }
                        },
                        disabled: text_content.read().is_empty(),
                        style: {
                            let is_disabled = text_content.read().is_empty();
                            format!(
                                "background: {}; color: {}; padding: 10px 20px; border: none; border-radius: 6px; cursor: {}; font-size: 1em; opacity: {};",
                                if is_disabled { &theme.surface } else { &theme.accent },
                                if is_disabled { &theme.text_secondary } else { "white" },
                                if is_disabled { "not-allowed" } else { "pointer" },
                                if is_disabled { "0.5" } else { "1" }
                            )
                        },
                        "Start Reading"
                    }
                }
            }
        }
    }
}