use dioxus::prelude::*;
use crate::components::{
    ProgressBar, TopControls, MainContent
};
use crate::components::features::vocabulary::WordMeaningsStyles;
use crate::components::features::gallery::image_gallery_styles::ImageGalleryStyles;
use crate::components::layout::AppLayout;
use crate::components::features::navigation::KeyboardHandler;
use crate::components::features::modals::ModalManager;
use crate::hooks::{use_app_state, use_word_meaning_effect};

#[component]
pub fn App() -> Element {
    let mut app_state = use_app_state();
    
    // Global image cache context
    let image_cache = use_signal(std::collections::HashMap::<String, crate::services::ImageFetchState>::new);
    use_context_provider(|| image_cache);
    
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
        ImageGalleryStyles { theme: app_state.theme.clone() }
        
        // Top-level controls
        TopControls {
            theme_mode: app_state.theme_mode,
            theme: app_state.theme.clone(),
            known_words_count: app_state.known_words_count(),
            sentence_count: app_state.floating_button_count(),
            on_theme_toggle: move |_| theme_state.toggle_theme(),
            on_known_words_click: move |_| known_words_state.show_known_words_modal(),
            on_add_text_click: move |_| input_modal_state.show_input_modal()
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
            }
        })}
        
        // Modals rendered outside main layout
        {ModalManager(app_state.clone())}
    }
}