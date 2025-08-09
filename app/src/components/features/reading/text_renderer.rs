use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::theme::Theme;
use crate::utils::{tokenize_text_for_clicks, is_word_token, generate_word_color_themed, find_phrase_matches};
use crate::components::ClickableWord;

#[component]
pub fn TextRenderer(
    text: String,
    word_meanings: Option<Vec<WordMeaning>>,
    theme: Theme,
    on_word_click: EventHandler<String>,
) -> Element {
    let tokens = tokenize_text_for_clicks(&text);
    let empty_vec = Vec::new();
    let word_meanings = word_meanings.as_ref().unwrap_or(&empty_vec);
    
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
                            ClickableWord {
                                key: "unhighlighted_{current_index}",
                                text: token.clone(),
                                index: current_index,
                                is_clickable: true,
                                style: "".to_string(),
                                on_click: on_word_click
                            }
                        });
                    } else {
                        elements.push(rsx! {
                            ClickableWord {
                                key: "non_word_{current_index}",
                                text: token.clone(),
                                index: current_index,
                                is_clickable: false,
                                style: "".to_string(),
                                on_click: on_word_click
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
                
                let color = generate_word_color_themed(&span_text, &theme);
                let style = format!("color: {color}; font-weight: 600;");
                
                // Render the entire span as one highlighted unit
                let span_tokens = if span.end_index < tokens.len() {
                    &tokens[span.start_index..=span.end_index]
                } else {
                    &tokens[span.start_index..]
                };
                let span_display_text = span_tokens.iter().map(|token| token.as_str()).collect::<String>();
                
                elements.push(rsx! {
                    span {
                        key: "highlighted_{span.start_index}_{span.end_index}",
                        style: "{style}; cursor: pointer; user-select: none; -webkit-user-select: none; -moz-user-select: none; -ms-user-select: none;",
                        ondoubleclick: {
                            let span_text_clone = span_text.clone();
                            let on_word_click_clone = on_word_click.clone();
                            move |_| on_word_click_clone.call(span_text_clone.clone())
                        },
                        "{span_display_text}"
                    }
                });
                
                current_index = span.end_index + 1;
            }
            
            // Render remaining non-highlighted tokens
            while current_index < tokens.len() {
                let token = &tokens[current_index];
                if is_word_token(token) {
                    elements.push(rsx! {
                        ClickableWord {
                            key: "final_unhighlighted_{current_index}",
                            text: token.clone(),
                            index: current_index,
                            is_clickable: true,
                            style: "".to_string(),
                            on_click: on_word_click
                        }
                    });
                } else {
                    elements.push(rsx! {
                        ClickableWord {
                            key: "final_non_word_{current_index}",
                            text: token.clone(),
                            index: current_index,
                            is_clickable: false,
                            style: "".to_string(),
                            on_click: on_word_click
                        }
                    });
                }
                current_index += 1;
            }
            
            elements.into_iter()
        }
    }
}
