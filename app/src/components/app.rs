use dioxus::prelude::*;
use crate::components::{
    FloatingButton, ProgressBar, ThemeToggle, KnownWordsCounter,
    WordMeaningsStyles, MainContent
};
use crate::components::layout::AppLayout;
use crate::components::features::navigation::KeyboardHandler;
use crate::components::features::modals::ModalManager;
use crate::hooks::{use_app_state, use_word_meaning_effect};

#[component]
pub fn App() -> Element {
    let mut app_state = use_app_state();
    
    // Handle word meaning effects
    use_word_meaning_effect(&mut app_state);
    
    // Clone for closures
    let mut theme_state = app_state.clone();
    let mut known_words_state = app_state.clone();
    let mut input_modal_state = app_state.clone();
    
    rsx! {
        // Global styles
        style { "body {{ margin: 0; padding: 0; background: {app_state.theme.background}; color: {app_state.theme.text_primary}; }}" }
        style { {include_str!("../../assets/fonts/spectral.css")} }
        WordMeaningsStyles { theme: app_state.theme.clone() }
        
        // Top-level controls
        ThemeToggle { 
            theme_mode: app_state.theme_mode, 
            on_toggle: move |_| theme_state.toggle_theme()
        }
        
        KnownWordsCounter {
            count: app_state.known_words_count(),
            theme: app_state.theme.clone(),
            on_click: move |_| known_words_state.show_known_words_modal()
        }
        
        // Main app structure with keyboard navigation
        {KeyboardHandler(app_state.clone(), rsx! {
            AppLayout {
                theme: app_state.theme.clone(),
                
                ProgressBar { 
                    current: {
                        let (current, _) = app_state.progress_values();
                        current
                    },
                    total: {
                        let (_, total) = app_state.progress_values();
                        total
                    },
                    theme: app_state.theme.clone()
                }
                
                MainContent {
                    reading_state: app_state.reading_state,
                    vocabulary_state: app_state.vocabulary_state,
                    sentence_to_fetch: app_state.sentence_to_fetch,
                    word_to_fetch: app_state.word_to_fetch,
                    encounter_tracked_sentences: app_state.encounter_tracked_sentences,
                    promotion_notification: app_state.promotion_notification,
                    theme: app_state.theme.clone(),
                }
                
                if app_state.should_show_floating_button() {
                    FloatingButton {
                        count: app_state.floating_button_count(),
                        onclick: move |_| input_modal_state.show_input_modal()
                    }
                }
            }
        })}
        
        // Modals rendered outside main layout
        {ModalManager(app_state.clone())}
    }
}