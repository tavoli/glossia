use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::utils::highlight_words_in_text;

#[component]
pub fn ReadingContainer(
    original: Option<String>,
    simplified: Option<String>,
    is_loading: bool,
    words: Option<Vec<WordMeaning>>,
    on_next: EventHandler<()>, 
    on_prev: EventHandler<()>
) -> Element {
    rsx! {
        div {
            class: "reading-container",
            style: "background: white; padding: 40px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.08); width: 80%; max-width: 700px; text-align: center; z-index: 20;",
            
            div {
                class: "original-text",
                style: "font-size: 1.5em; color: #333; line-height: 1.4;",
                dangerous_inner_html: if let Some(ref text) = original {
                    if let Some(ref word_meanings) = words {
                        highlight_words_in_text(text, word_meanings)
                    } else {
                        text.clone()
                    }
                } else {
                    "Paste text to start reading.".to_string()
                }
            }
            
            div {
                class: "simplified-text",
                style: "margin-top: 20px; font-size: 1.2em; color: #555; min-height: 50px; line-height: 1.4;",
                
                if is_loading {
                    div { class: "loading-indicator", "Loading..." }
                } else if let Some(text) = simplified {
                    div {
                        dangerous_inner_html: if let Some(ref word_meanings) = words {
                            highlight_words_in_text(&text, word_meanings)
                        } else {
                            text
                        }
                    }
                }
            }
            
            div {
                class: "navigation-controls",
                style: "margin-top: 30px;",
                button { onclick: move |_| on_prev.call(()), "Previous" }
                button { onclick: move |_| on_next.call(()), "Next" }
            }
        }
    }
}
