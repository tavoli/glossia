use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn ClipboardToast(
    theme: Theme,
    clipboard_text: String,
    on_read: EventHandler<()>,
    on_dismiss: EventHandler<()>,
) -> Element {
    let truncated_text = if clipboard_text.len() > 50 {
        format!("{}...", &clipboard_text[..50])
    } else {
        clipboard_text.clone()
    };
    
    rsx! {
        style {
            {format!("
            @keyframes slideUp {{
                from {{
                    transform: translate(-50%, 100%);
                    opacity: 0;
                }}
                to {{
                    transform: translate(-50%, 0);
                    opacity: 1;
                }}
            }}
            
            @keyframes slideDown {{
                from {{
                    transform: translate(-50%, 0);
                    opacity: 1;
                }}
                to {{
                    transform: translate(-50%, 100%);
                    opacity: 0;
                }}
            }}
            
            .clipboard-toast {{
                animation: slideUp 0.3s ease-out forwards;
            }}
            
            .clipboard-toast.dismissing {{
                animation: slideDown 0.3s ease-out forwards;
            }}
            
            .clipboard-toast button {{
                transition: all 0.2s ease;
            }}
            
            .clipboard-toast button:hover {{
                transform: translateY(-1px);
                box-shadow: 0 4px 12px rgba(0,0,0,0.15);
            }}
            ")}
        }
        
        div {
            class: "clipboard-toast",
            style: format!(
                "
                position: fixed;
                bottom: 40px;
                left: 50%;
                background: {};
                border: 1px solid {};
                border-radius: 12px;
                padding: 16px 20px;
                box-shadow: 0 4px 20px rgba(0,0,0,0.15);
                z-index: 1000;
                display: flex;
                align-items: center;
                gap: 16px;
                max-width: 500px;
                min-width: 400px;
                backdrop-filter: blur(10px);
                ",
                if theme.mode == crate::theme::ThemeMode::Light { 
                    "rgba(255, 255, 255, 0.95)" 
                } else { 
                    "rgba(30, 30, 30, 0.95)" 
                },
                theme.border
            ),
            
            // Clipboard icon
            div {
                style: format!(
                    "
                    width: 36px;
                    height: 36px;
                    border-radius: 8px;
                    background: {};
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    flex-shrink: 0;
                    ",
                    if theme.mode == crate::theme::ThemeMode::Light { 
                        "linear-gradient(135deg, #e3f2fd 0%, #bbdefb 100%)" 
                    } else { 
                        "linear-gradient(135deg, #1e3a5f 0%, #2c5282 100%)" 
                    }
                ),
                "📋"
            }
            
            // Text content
            div {
                style: "flex: 1; min-width: 0;",
                
                div {
                    style: format!(
                        "color: {}; font-size: 0.9em; font-weight: 600; margin-bottom: 4px;",
                        theme.text_primary
                    ),
                    "New text in clipboard"
                }
                
                div {
                    style: format!(
                        "color: {}; font-size: 0.85em; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                        theme.text_secondary
                    ),
                    "{truncated_text}"
                }
            }
            
            // Action buttons - user must explicitly choose
            div {
                style: "display: flex; gap: 8px; flex-shrink: 0;",
                
                button {
                    onclick: move |_| on_dismiss.call(()),
                    style: format!(
                        "
                        background: transparent;
                        color: {};
                        padding: 8px 16px;
                        border: none;
                        border-radius: 6px;
                        cursor: pointer;
                        font-size: 0.9em;
                        font-weight: 500;
                        ",
                        theme.text_secondary
                    ),
                    "Dismiss"
                }
                
                button {
                    onclick: move |_| on_read.call(()),
                    style: format!(
                        "
                        background: {};
                        color: white;
                        padding: 8px 20px;
                        border: none;
                        border-radius: 6px;
                        cursor: pointer;
                        font-size: 0.9em;
                        font-weight: 600;
                        ",
                        theme.accent
                    ),
                    "Read from Clipboard"
                }
            }
        }
    }
}