pub mod common;
pub mod features;
pub mod layout;

// Legacy flat components (to be migrated)
pub mod floating_button;
pub mod progress_bar;
pub mod reading_container;
pub mod styles;
pub mod text_input_modal;
pub mod theme_toggle;
pub mod word_meanings;
pub mod error_display;
pub mod image_gallery;
pub mod word_meaning_item;
pub mod known_words_counter;
pub mod app;
pub mod main_content;
pub mod clickable_word;
pub mod token_highlighter;
pub mod text_renderer;
pub mod navigation_controls;

// Public exports
pub use floating_button::FloatingButton;
pub use progress_bar::ProgressBar;
pub use reading_container::ReadingContainer;
pub use styles::WordMeaningsStyles;
pub use text_input_modal::TextInputModal;
pub use theme_toggle::ThemeToggle;
pub use word_meanings::WordMeanings;
pub use error_display::ErrorDisplay;
pub use image_gallery::ImageGallery;
pub use word_meaning_item::WordMeaningItem;
pub use known_words_counter::KnownWordsCounter;
pub use features::KnownWordsModal;
pub use app::App;
pub use main_content::MainContent;
pub use clickable_word::ClickableWord;
pub use text_renderer::TextRenderer;
pub use navigation_controls::NavigationControls;

// New organized components
pub use layout::AppLayout;
pub use features::{ModalManager, KeyboardHandler, LoadingState, ContentDisplay, SentenceProcessor};
pub use features::vocabulary::WordMeaningsStyles as WordMeaningsStylesNew;
