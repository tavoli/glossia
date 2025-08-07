use dioxus::prelude::*;
use glossia_book_reader::ReadingState;

/// Custom hook for managing reading state
pub fn use_reading_state() -> Signal<ReadingState> {
    use_signal(|| ReadingState::new().expect("Failed to initialize reading state"))
}

/// Custom hook for handling navigation - returns closures that can be converted to EventHandlers
pub fn use_navigation(reading_state: Signal<ReadingState>) -> (impl FnMut() + 'static, impl FnMut() + 'static) {
    let on_next = {
        let mut reading_state = reading_state.clone();
        move || {
            reading_state.write().next();
        }
    };
    
    let on_prev = {
        let mut reading_state = reading_state.clone();
        move || {
            reading_state.write().previous();
        }
    };
    
    (on_next, on_prev)
}


