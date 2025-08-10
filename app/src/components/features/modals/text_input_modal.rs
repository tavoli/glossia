use dioxus::prelude::*;
use crate::components::common::modals::Modal;
use crate::theme::{use_theme, Theme};

#[component]
pub fn TextInputModal(onsubmit: EventHandler<String>, onclose: EventHandler<()>) -> Element {
    let mut text_content = use_signal(String::new);
    let theme_mode = use_theme();
    let theme = Theme::from_mode(*theme_mode.read());
    
    let scrollbar_thumb_color = if theme.mode == crate::theme::ThemeMode::Light { 
        "rgba(0,0,0,0.2)" 
    } else { 
        "rgba(255,255,255,0.2)" 
    };
    
    let scrollbar_thumb_hover = if theme.mode == crate::theme::ThemeMode::Light { 
        "rgba(0,0,0,0.3)" 
    } else { 
        "rgba(255,255,255,0.3)" 
    };
    
    rsx! {
        style {
            {format!("
            .text-input-modal textarea::-webkit-scrollbar {{
                width: 6px;
            }}
            
            .text-input-modal textarea::-webkit-scrollbar-track {{
                background: transparent;
                border-radius: 3px;
            }}
            
            .text-input-modal textarea::-webkit-scrollbar-thumb {{
                background: {};
                border-radius: 3px;
            }}
            
            .text-input-modal textarea::-webkit-scrollbar-thumb:hover {{
                background: {};
            }}
            
            .text-input-modal textarea {{
                scrollbar-width: thin;
                scrollbar-color: {} transparent;
            }}
            
            .text-input-modal button {{
                transition: all 0.2s ease;
            }}
            
            .text-input-modal button:hover:not(:disabled) {{
                transform: translateY(-1px);
                box-shadow: 0 2px 8px rgba(0,0,0,0.15);
            }}
            ", scrollbar_thumb_color, scrollbar_thumb_hover, scrollbar_thumb_color)}
        }
        
        Modal {
            theme: theme.clone(),
            on_close: Some(onclose.clone()),
            max_width: Some("700px".to_string()),
            
            div {
                class: "text-input-modal",
                style: format!(
                    "padding: 32px; background: {}; border-radius: 12px;",
                    if theme.mode == crate::theme::ThemeMode::Light { 
                        "linear-gradient(135deg, #ffffff 0%, #fafafa 100%)" 
                    } else { 
                        "linear-gradient(135deg, #2d2d2d 0%, #262626 100%)" 
                    }
                ),
                
                // Header with icon
                div {
                    style: "display: flex; align-items: center; gap: 12px; margin-bottom: 24px;",
                    
                    div {
                        style: format!(
                            "width: 40px; height: 40px; border-radius: 10px; background: {}; display: flex; align-items: center; justify-content: center; font-size: 1.2em;",
                            if theme.mode == crate::theme::ThemeMode::Light { 
                                "linear-gradient(135deg, #e3f2fd 0%, #bbdefb 100%)" 
                            } else { 
                                "linear-gradient(135deg, #1e3a5f 0%, #2c5282 100%)" 
                            }
                        ),
                        "📖"
                    }
                    
                    h2 {
                        style: "margin: 0; color: {theme.text_primary}; font-size: 1.6em; font-weight: 600;",
                        "Add Text to Read"
                    }
                }
                
                // Subtitle
                p {
                    style: "margin: 0 0 20px 0; color: {theme.text_secondary}; font-size: 0.95em;",
                    "Paste or type the text you want to read and learn from"
                }
                
                textarea {
                    class: "text-input",
                    style: format!(
                        "
                        min-height: 280px; 
                        border: 2px solid {}; 
                        border-radius: 10px; 
                        margin-bottom: 24px; 
                        width: 100%; 
                        resize: vertical; 
                        font-family: 'Spectral', serif;
                        font-size: 1.05em;
                        line-height: 1.6;
                        padding: 16px;
                        background: {};
                        color: {};
                        box-sizing: border-box;
                        transition: all 0.2s ease;
                        outline: none;
                        ",
                        if theme.mode == crate::theme::ThemeMode::Light { "#e0e0e0" } else { "#404040" },
                        theme.background,
                        theme.text_primary
                    ),
                    placeholder: "Paste your text here...",
                    oninput: move |e| text_content.set(e.value()),
                    value: "{text_content}",
                    onfocus: move |_| {},
                    onkeydown: move |e| {
                        // Prevent Escape from bubbling when in textarea
                        if e.key() == Key::Escape {
                            e.stop_propagation();
                        }
                    }
                }
                
                // Character count
                div {
                    style: "margin-bottom: 20px; color: {theme.text_secondary}; font-size: 0.85em;",
                    "{text_content.read().len()} characters"
                }
                
                div {
                    style: "display: flex; justify-content: space-between; align-items: center;",
                    
                    // Left side hint
                    div {
                        style: "color: {theme.text_secondary}; font-size: 0.85em;",
                        "Tip: Use Ctrl+V to paste"
                    }
                    
                    // Right side buttons
                    div {
                        style: "display: flex; gap: 12px;",
                        
                        button {
                            onclick: move |_| onclose.call(()),
                            style: format!(
                                "
                                background: transparent; 
                                color: {}; 
                                padding: 12px 24px; 
                                border: 2px solid {}; 
                                border-radius: 8px; 
                                cursor: pointer;
                                font-size: 1em;
                                font-weight: 500;
                                ",
                                theme.text_primary,
                                theme.border
                            ),
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
                                    "
                                    background: {}; 
                                    color: white; 
                                    padding: 12px 32px; 
                                    border: none; 
                                    border-radius: 8px; 
                                    cursor: {}; 
                                    font-size: 1em; 
                                    font-weight: 600;
                                    opacity: {};
                                    ",
                                    if is_disabled { 
                                        if theme.mode == crate::theme::ThemeMode::Light { "#cccccc" } else { "#555555" }
                                    } else { 
                                        &theme.accent 
                                    },
                                    if is_disabled { "not-allowed" } else { "pointer" },
                                    if is_disabled { "0.6" } else { "1" }
                                )
                            },
                            "Start Reading"
                        }
                    }
                }
            }
        }
    }
}