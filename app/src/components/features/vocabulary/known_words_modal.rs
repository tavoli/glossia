use dioxus::prelude::*;
use crate::theme::Theme;
use crate::components::common::modals::Modal;
use super::{ModalHeader, SearchBar, WordGrid, EmptyState};
use super::known_words_modal_styles::KnownWordsModalStyles;

#[component]
pub fn KnownWordsModal(
    words: Vec<String>,
    theme: Theme,
    on_close: EventHandler<()>,
    on_remove_word: EventHandler<String>,
) -> Element {
    let search_query = use_signal(|| String::new());
    let words_clone = words.clone();
    let styles = KnownWordsModalStyles::new(&theme);
    
    let filtered_words = use_memo(move || {
        let query = search_query.read().to_lowercase();
        if query.is_empty() {
            words_clone.clone()
        } else {
            words_clone.iter()
                .filter(|word| word.to_lowercase().contains(&query))
                .cloned()
                .collect()
        }
    });

    rsx! {
        Modal {
            theme: theme.clone(),
            on_close: Some(on_close.clone()),
            max_width: Some("600px".to_string()),
            
            ModalHeader {
                title: "Known Words".to_string(),
                count: words.len(),
                theme: theme.clone(),
                on_close: on_close.clone(),
            }
            
            SearchBar {
                search_query: search_query,
                theme: theme.clone(),
            }
            
            div {
                class: "modal-body",
                style: "{styles.body()}",
                
                if filtered_words.read().is_empty() {
                    EmptyState {
                        is_searching: !search_query.read().is_empty(),
                        theme: theme.clone(),
                    }
                } else {
                    WordGrid {
                        words: filtered_words.read().clone(),
                        theme: theme.clone(),
                        on_remove_word: on_remove_word.clone(),
                    }
                }
            }
            
            div {
                class: "modal-footer",
                style: "{styles.footer()}",
                
                button {
                    style: "{styles.action_button()}",
                    onclick: move |_| on_close.call(()),
                    "Close"
                }
            }
        }
    }
}