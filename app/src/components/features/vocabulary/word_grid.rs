use dioxus::prelude::*;
use crate::components::features::vocabulary::known_words_modal_styles::KnownWordsModalStyles;
use crate::theme::Theme;

#[component]
pub fn WordGrid(
    words: Vec<String>,
    theme: Theme,
    on_remove_word: EventHandler<String>,
) -> Element {
    let styles = KnownWordsModalStyles::new(&theme);
    
    rsx! {
        div {
            class: "words-grid",
            style: "{styles.words_grid()}",
            
            for word in words.iter() {
                WordItem {
                    word: word.clone(),
                    theme: theme.clone(),
                    on_remove: on_remove_word.clone(),
                }
            }
        }
    }
}

#[component]
fn WordItem(
    word: String,
    theme: Theme,
    on_remove: EventHandler<String>,
) -> Element {
    let styles = KnownWordsModalStyles::new(&theme);
    
    rsx! {
        div {
            key: "{word}",
            class: "word-item",
            style: "{styles.word_item()}",
            
            span {
                style: "{styles.word_text()}",
                "{word}"
            }
            
            button {
                style: "{styles.remove_button()}",
                onclick: {
                    let word = word.clone();
                    move |_| on_remove.call(word.clone())
                },
                title: "Remove word",
                "✕"
            }
        }
    }
}