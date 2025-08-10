pub mod common;
pub mod features;
pub mod layout;

// Legacy flat components still remaining
pub mod floating_button;
pub mod progress_bar;
pub mod theme_toggle;
pub mod error_display;
pub mod known_words_counter;
pub mod top_controls;
pub mod app;
pub mod main_content;

// Public exports - organized by feature
// From features/vocabulary
pub use features::vocabulary::{
    WordMeanings, WordMeaningItem
};

// From features/reading  
pub use features::reading::{
    ClickableWord, TextRenderer, ReadingContainer, NavigationControls
};

// From features/gallery
pub use features::gallery::ImageGallery;

// From features/modals

// From features/navigation

// From layout

// From common

// Legacy components
pub use progress_bar::ProgressBar;
pub use error_display::ErrorDisplay;
pub use top_controls::TopControls;
pub use app::App;
pub use main_content::MainContent;