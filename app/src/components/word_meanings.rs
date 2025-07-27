use dioxus::prelude::*;
use glossia_shared::WordMeaning;
use crate::utils::generate_word_color;

#[component]
pub fn WordMeanings(words: Vec<WordMeaning>) -> Element {
    if words.is_empty() {
        return None;
    }

    rsx! {
        div {
            class: "word-meanings-container",
            style: "margin-top: 24px; width: 80%; padding: 40px; background: #ffffff; max-width: 700px;",
            
            div {
                class: "meanings-list",
                style: "
                    background: #ffffff;
                    overflow: hidden;
                ",
                for (index, word_meaning) in words.iter().enumerate() {
                    div {
                        class: "meaning-item",
                        style: format!("
                            display: flex;
                            align-items: flex-start;
                            padding: 12px 16px;
                            border-bottom: {};
                            transition: background-color 0.15s ease;
                        ", if index < words.len() - 1 { "1px solid #f1f3f4" } else { "none" }),
                        
                        div {
                            class: "word-label",
                            style: format!("
                                background: {};
                                color: white;
                                padding: 3px 8px;
                                border-radius: 16px;
                                font-size: 0.8em;
                                font-weight: 600;
                                min-width: fit-content;
                                margin-right: 12px;
                                margin-top: 2px;
                                box-shadow: 0 1px 3px rgba(0,0,0,0.15);
                            ", generate_word_color(&word_meaning.word)),
                            "{word_meaning.word}"
                        }
                        
                        div {
                            class: "meaning-definition",
                            style: "
                                color: #495057;
                                line-height: 1.4;
                                font-size: 0.9em;
                                flex: 1;
                            ",
                            "{word_meaning.meaning}"
                        }
                    }
                }
            }
        }
    }
}
