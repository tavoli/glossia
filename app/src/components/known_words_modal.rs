use dioxus::prelude::*;
use crate::theme::Theme;

#[component]
pub fn KnownWordsModal(
    words: Vec<String>,
    theme: Theme,
    on_close: EventHandler<()>,
    on_remove_word: EventHandler<String>,
) -> Element {
    let mut search_query = use_signal(|| String::new());
    let words_clone = words.clone();
    
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
        div {
            class: "modal-overlay",
            style: "
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.5);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 1000;
                backdrop-filter: blur(5px);
            ",
            onclick: move |_| {
                // Close modal when clicking on overlay
                on_close.call(());
            },
            
            div {
                class: "modal-content",
                style: "
                    background: {theme.surface};
                    border-radius: 12px;
                    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
                    width: 90%;
                    max-width: 600px;
                    max-height: 80vh;
                    display: flex;
                    flex-direction: column;
                    border: 1px solid {theme.border};
                ",
                onclick: |_| {},
                
                // Header
                div {
                    class: "modal-header",
                    style: "
                        padding: 20px;
                        border-bottom: 1px solid {theme.border};
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                    ",
                    
                    h2 {
                        style: "
                            margin: 0;
                            color: {theme.text_primary};
                            font-size: 1.5em;
                            font-weight: 600;
                        ",
                        "Known Words ({words.len()})"
                    }
                    
                    button {
                        style: "
                            background: none;
                            border: none;
                            font-size: 1.5em;
                            cursor: pointer;
                            color: {theme.text_secondary};
                            padding: 5px;
                            border-radius: 4px;
                            transition: background-color 0.2s ease;
                        ",
                        onclick: move |_| on_close.call(()),
                        title: "Close",
                        "×"
                    }
                }
                
                // Search bar
                div {
                    class: "modal-search",
                    style: "
                        padding: 20px;
                        border-bottom: 1px solid {theme.border};
                    ",
                    
                    input {
                        style: "
                            width: 100%;
                            padding: 12px;
                            border: 1px solid {theme.border};
                            border-radius: 6px;
                            background: {theme.background};
                            color: {theme.text_primary};
                            font-size: 1em;
                            box-sizing: border-box;
                        ",
                        r#type: "text",
                        placeholder: "Search words...",
                        value: "{search_query}",
                        oninput: move |e| {
                            search_query.set(e.value());
                        }
                    }
                }
                
                // Words list
                div {
                    class: "modal-body",
                    style: "
                        flex: 1;
                        overflow-y: auto;
                        padding: 20px;
                        max-height: 400px;
                    ",
                    
                    if filtered_words.read().is_empty() {
                        div {
                            style: "
                                text-align: center;
                                color: {theme.text_secondary};
                                padding: 40px;
                                font-style: italic;
                            ",
                            if search_query.read().is_empty() {
                                "No known words yet. Start reading and double-click words to add them!"
                            } else {
                                "No words found matching your search."
                            }
                        }
                    } else {
                        div {
                            class: "words-grid",
                            style: "
                                display: grid;
                                grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
                                gap: 12px;
                            ",
                            
                            for word in filtered_words.read().iter() {
                                div {
                                    key: "{word}",
                                    class: "word-item",
                                    style: "
                                        background: {theme.background};
                                        border: 1px solid {theme.border};
                                        border-radius: 8px;
                                        padding: 12px;
                                        display: flex;
                                        justify-content: space-between;
                                        align-items: center;
                                        transition: all 0.2s ease;
                                    ",
                                    onmouseenter: |_| {},
                                    onmouseleave: |_| {},
                                    
                                    span {
                                        style: "
                                            color: {theme.text_primary};
                                            font-weight: 500;
                                            flex: 1;
                                            text-align: left;
                                        ",
                                        "{word}"
                                    }
                                    
                                    button {
                                        style: "
                                            background: {theme.error};
                                            border: none;
                                            color: white;
                                            border-radius: 4px;
                                            padding: 4px 8px;
                                            cursor: pointer;
                                            font-size: 0.8em;
                                            transition: opacity 0.2s ease;
                                        ",
                                        onclick: {
                                            let word = word.clone();
                                            move |_| on_remove_word.call(word.clone())
                                        },
                                        title: "Remove word",
                                        "✕"
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Footer
                div {
                    class: "modal-footer",
                    style: "
                        padding: 20px;
                        border-top: 1px solid {theme.border};
                        display: flex;
                        justify-content: flex-end;
                        gap: 10px;
                    ",
                    
                    button {
                        style: "
                            background: {theme.accent};
                            color: white;
                            border: none;
                            padding: 10px 20px;
                            border-radius: 6px;
                            cursor: pointer;
                            font-size: 1em;
                            transition: opacity 0.2s ease;
                        ",
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
            }
        }
    }
}
