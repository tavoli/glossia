use dioxus::prelude::*;
use crate::theme::{Theme, ThemeMode};

#[component]
pub fn TopControls(
    theme_mode: Signal<ThemeMode>,
    theme: Theme,
    known_words_count: usize,
    sentence_count: usize,
    on_theme_toggle: EventHandler<()>,
    on_known_words_click: EventHandler<()>,
    on_add_text_click: EventHandler<()>,
) -> Element {
    let display_count = if known_words_count > 999 {
        "999+".to_string()
    } else {
        known_words_count.to_string()
    };

    rsx! {
        div {
            class: "top-controls",
            style: "
                position: fixed;
                top: 20px;
                right: 20px;
                z-index: 100;
                display: flex;
                flex-direction: row;
                gap: 12px;
                align-items: center;
            ",
            
            // Add text button
            button {
                class: "add-text-btn",
                style: "
                    position: relative;
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
                    font-size: 0.9em;
                    font-weight: 600;
                ",
                onclick: move |_| on_add_text_click.call(()),
                title: "Add new text ({sentence_count} sentences)",
                
                span {
                    style: "
                        line-height: 1;
                        text-align: center;
                        overflow: hidden;
                        text-overflow: ellipsis;
                        white-space: nowrap;
                        max-width: 35px;
                    ",
                    "{sentence_count}"
                }
                
                // Modern icon overlay
                div {
                    style: "
                        position: absolute;
                        top: -4px;
                        right: -4px;
                        background: {theme.accent};
                        border-radius: 50%;
                        width: 18px;
                        height: 18px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-size: 14px;
                        font-weight: bold;
                        box-shadow: 0 1px 3px rgba(0,0,0,0.3);
                    ",
                    "+"
                }
            }
            
            // Known words counter button
            button {
                class: "known-words-btn",
                style: "
                    position: relative;
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
                    font-size: 0.9em;
                    font-weight: 600;
                ",
                onclick: move |_| on_known_words_click.call(()),
                title: "View known words ({known_words_count})",
                
                span {
                    style: "
                        line-height: 1;
                        text-align: center;
                        overflow: hidden;
                        text-overflow: ellipsis;
                        white-space: nowrap;
                        max-width: 35px;
                    ",
                    "{display_count}"
                }
                
                // Modern icon overlay for known words
                div {
                    style: "
                        position: absolute;
                        top: -4px;
                        right: -4px;
                        background: {theme.accent};
                        border-radius: 50%;
                        width: 18px;
                        height: 18px;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: white;
                        font-size: 12px;
                        box-shadow: 0 1px 3px rgba(0,0,0,0.3);
                    ",
                    "✓"
                }
            }
            
            // Theme toggle button
            button {
                class: "theme-toggle-btn",
                style: "
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
                onclick: move |_| on_theme_toggle.call(()),
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
}