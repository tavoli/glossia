use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::theme::Theme;
use crate::utils::{tokenize_text_for_clicks, is_word_token, generate_word_color, find_phrase_matches};

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
    // Helper function to render clickable text with phrase support
    let render_clickable_text = move |text: &str, word_meanings: Option<&Vec<WordMeaning>>| -> Element {
        let tokens = tokenize_text_for_clicks(text);
        let empty_vec = Vec::new();
        let word_meanings = word_meanings.unwrap_or(&empty_vec);
        
        // Find all highlight spans (both phrases and words)
        let highlight_spans = find_phrase_matches(&tokens, word_meanings);
        
        rsx! {
            {
                let mut current_index = 0;
                let mut elements = Vec::new();
                
                for span in &highlight_spans {
                    // Render non-highlighted tokens before this span
                    while current_index < span.start_index {
                        let token = &tokens[current_index];
                        if is_word_token(token) {
                            elements.push(rsx! {
                                span {
                                    key: "unhighlighted_{current_index}",
                                    style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none; cursor: pointer;",
                                    ondoubleclick: {
                                        let token_clone = token.clone();
                                        let on_word_click_clone = on_word_click.clone();
                                        move |_| on_word_click_clone.call(token_clone.clone())
                                    },
                                    "{token}"
                                }
                            });
                        } else {
                            elements.push(rsx! {
                                span {
                                    key: "unhighlighted_{current_index}",
                                    style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                                    "{token}"
                                }
                            });
                        }
                        current_index += 1;
                    }
                    
                    // Render the highlighted span
                    let span_text = if span.is_phrase && span.end_index < tokens.len() {
                        // For phrases, use the original phrase text for color generation
                        word_meanings.iter()
                            .find(|wm| wm.is_phrase && 
                                wm.word.split_whitespace()
                                    .zip(tokens[span.start_index..=span.end_index].iter().filter(|t| is_word_token(t)))
                                    .all(|(phrase_word, token)| phrase_word.to_lowercase() == token.to_lowercase()))
                            .map(|wm| wm.word.clone())
                            .unwrap_or_else(|| span.text.clone())
                    } else {
                        span.text.clone()
                    };
                    
                    let color = generate_word_color(&span_text);
                    let font_weight = "600";
                    
                    // Render the entire span as one highlighted unit
                    let span_tokens = if span.end_index < tokens.len() {
                        &tokens[span.start_index..=span.end_index]
                    } else {
                        &tokens[span.start_index..]
                    };
                    let span_key = format!("highlighted_{}_{}", span.start_index, span.end_index);
                    
                    elements.push(rsx! {
                        span {
                            key: "{span_key}",
                            style: "color: {color}; font-weight: {font_weight}; cursor: pointer; user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                            ondoubleclick: {
                                let span_text_clone = span_text.clone();
                                let on_word_click_clone = on_word_click.clone();
                                move |_| on_word_click_clone.call(span_text_clone.clone())
                            },
                            {span_tokens.iter().map(|token| token.as_str()).collect::<String>()}
                        }
                    });
                    
                    current_index = span.end_index + 1;
                }
                
                // Render remaining non-highlighted tokens
                while current_index < tokens.len() {
                    let token = &tokens[current_index];
                    if is_word_token(token) {
                        elements.push(rsx! {
                            span {
                                key: "final_unhighlighted_{current_index}",
                                style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none; cursor: pointer;",
                                ondoubleclick: {
                                    let token_clone = token.clone();
                                    let on_word_click_clone = on_word_click.clone();
                                    move |_| on_word_click_clone.call(token_clone.clone())
                                },
                                "{token}"
                            }
                        });
                    } else {
                        elements.push(rsx! {
                            span {
                                key: "final_unhighlighted_{current_index}",
                                style: "user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                                "{token}"
                            }
                        });
                    }
                    current_index += 1;
                }
                
                elements.into_iter()
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
