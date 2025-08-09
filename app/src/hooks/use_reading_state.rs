use dioxus::prelude::*;
use glossia_reading_engine::ReadingEngine;

/// Custom hook for managing reading state
pub fn use_reading_state() -> Signal<ReadingEngine> {
    use_signal(|| ReadingEngine::new().expect("Failed to initialize reading engine"))
}

/// Custom hook for handling navigation - returns closures that can be converted to EventHandlers
#[allow(dead_code)]
pub fn use_navigation(reading_state: Signal<ReadingEngine>) -> (impl FnMut() + 'static, impl FnMut() + 'static) {
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


