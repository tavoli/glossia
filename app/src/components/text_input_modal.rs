use dioxus::prelude::*;

#[component]
pub fn TextInputModal(onsubmit: EventHandler<String>, onclose: EventHandler<()>) -> Element {
    let mut text_content = use_signal(String::new);
    
    rsx! {
        div {
            class: "modal-overlay",
            style: "position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center; z-index: 200;",
            onkeydown: move |e| {
                if e.key() == Key::Escape {
                    onclose.call(());
                }
            },
            onclick: move |_| {
                // Close modal when clicking on overlay
                onclose.call(());
            },
            
            div {
                class: "modal-content",
                style: "background: white; padding: 30px; border-radius: 8px; width: 80%; max-width: 600px;",
                onclick: move |e| {
                    // Prevent event bubbling to overlay
                    e.stop_propagation();
                },
                
                textarea {
                    class: "text-input",
                    style: "min-height: 200px; border: 1px solid #ddd; border-radius: 5px; margin-bottom: 20px; width: 100%; resize: vertical; font-family: inherit;",
                    placeholder: "Paste your text here...",
                    oninput: move |e| text_content.set(e.value()),
                    value: "{text_content}"
                }
                
                button {
                    onclick: move |_| {
                        onsubmit.call(text_content())
                    },
                    style: "background: #4a90e2; color: white; padding: 10px 20px; border: none; border-radius: 5px; cursor: pointer;",
                    "Start Reading"
                }
            }
        }
    }
}
