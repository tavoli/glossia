use dioxus::prelude::*;
use crate::theme::{ThemeMode, Theme};

#[component]
pub fn ThemeToggle(theme_mode: Signal<ThemeMode>, on_toggle: EventHandler<()>) -> Element {
    let theme = Theme::from_mode(*theme_mode.read());
    
    rsx! {
        button {
            class: "theme-toggle",
            style: "
                position: fixed;
                top: 20px;
                right: 20px;
                z-index: 100;
                background: {theme.surface};
                border: 2px solid {theme.border};
                border-radius: 50%;
                width: 50px;
                height: 50px;
                display: flex;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                transition: all 0.3s ease;
                box-shadow: 0 2px 8px {theme.shadow};
                color: {theme.text_primary};
                font-size: 1.2em;
            ",
            onclick: move |_| on_toggle.call(()),
            title: match *theme_mode.read() {
                ThemeMode::Light => "Switch to dark mode",
                ThemeMode::Dark => "Switch to light mode",
            },
            match *theme_mode.read() {
                ThemeMode::Light => "🌙",
                ThemeMode::Dark => "☀️",
            }
        }
    }
}