use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::theme::Theme;
use crate::utils::{tokenize_text_for_clicks, is_word_token, generate_word_color};

#[component]
pub fn ReadingContainer(
    original: Option<String>,
    simplified: Option<String>,
    is_loading: bool,
    words: Option<Vec<WordMeaning>>,
    theme: Theme,
    on_next: EventHandler<()>, 
    on_prev: EventHandler<()>,
    on_word_click: EventHandler<String>
) -> Element {
    // Helper function to render clickable text
    let render_clickable_text = move |text: &str, word_meanings: Option<&Vec<WordMeaning>>| -> Element {
        let tokens = tokenize_text_for_clicks(text);
        let empty_vec = Vec::new();
        let word_meanings = word_meanings.unwrap_or(&empty_vec);
        
        rsx! {
            for (index, token) in tokens.iter().enumerate() {
                if is_word_token(token) {
                    // Check if this word has a meaning (for highlighting)
                    {
                        let is_highlighted = word_meanings.iter().any(|w| w.word.to_lowercase() == token.to_lowercase());
                        let color = if is_highlighted { generate_word_color(token) } else { "transparent".to_string() };
                        let text_color = if is_highlighted { "white" } else { &theme.text_primary };
                        let font_weight = if is_highlighted { "600" } else { "400" };
                        let padding = if is_highlighted { "3px 8px" } else { "0" };
                        let border_radius = if is_highlighted { "16px" } else { "0" };
                        let font_size = if is_highlighted { "0.95em" } else { "1em" };
                        let box_shadow = if is_highlighted { "0 1px 3px rgba(0,0,0,0.15)" } else { "none" };
                        
                        rsx! {
                            span {
                                key: "{index}",
                                style: "color: {text_color}; font-weight: {font_weight}; background: {color}; padding: {padding}; border-radius: {border_radius}; font-size: {font_size}; box-shadow: {box_shadow}; cursor: pointer; user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                                ondoubleclick: {
                                    let token_clone = token.clone();
                                    let on_word_click_clone = on_word_click.clone();
                                    move |_| on_word_click_clone.call(token_clone.clone())
                                },
                                "{token}"
                            }
                        }
                    }
                } else {
                    // Non-word token (spaces, punctuation)
                    span {
                        key: "{index}",
                        style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                        "{token}"
                    }
                }
            }
        }
    };

    rsx! {
        div {
            class: "reading-container",
            style: "background: {theme.surface}; padding: 40px; border-radius: 8px; box-shadow: 0 2px 10px {theme.shadow}; width: 80%; max-width: 700px; text-align: center; z-index: 20; border: 1px solid {theme.border};",
            
            div {
                class: "original-text",
                style: "font-size: 1.5em; color: {theme.text_primary}; line-height: 1.4;",
                if let Some(ref text) = original {
                    {render_clickable_text(text, words.as_ref())}
                } else {
                    "Paste text to start reading."
                }
            }
            
            div {
                class: "simplified-text",
                style: "margin-top: 20px; font-size: 1.2em; color: {theme.text_secondary}; min-height: 50px; line-height: 1.4;",
                
                if is_loading {
                    div { class: "loading-indicator", "Loading..." }
                } else if let Some(text) = simplified {
                    {render_clickable_text(&text, words.as_ref())}
                }
            }
            
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
}
