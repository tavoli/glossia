use dioxus::prelude::*;
use glossia_shared::ImageResult;
use std::collections::HashMap;
use crate::services::ImageFetchState;

#[component]
pub fn ImageGallery(
    word: String,
    image_cache: Signal<HashMap<String, ImageFetchState>>,
) -> Element {
    let image_state = image_cache.read().get(&word).cloned();
    
    match image_state {
        Some(ImageFetchState::Loading) | None => rsx! {
            div {
                class: "image-gallery",
                div {
                    class: "gallery-message loading",
                    span { class: "spinner", }
                    "Loading images..."
                }
            }
        },
        Some(ImageFetchState::Loaded(images)) => {
            if images.is_empty() {
                rsx! {
                    div {
                        class: "image-gallery",
                        div {
                            class: "gallery-message",
                            "No images found"
                        }
                    }
                }
            } else {
                rsx! {
                    div {
                        class: "image-gallery",
                        
                        div {
                            class: "gallery-header",
                            "Images for \"{word}\""
                        }
                        
                        div {
                            class: "images-grid",
                            for (index, image) in images.iter().enumerate() {
                                ImageThumbnail {
                                    key: "{word}-{index}",
                                    image: image.clone(),
                                }
                            }
                        }
                    }
                }
            }
        },
        Some(ImageFetchState::Error(error)) => rsx! {
            div {
                class: "image-gallery",
                div {
                    class: "gallery-message error",
                    "⚠ {error}"
                }
            }
        },
    }
}

#[component]
fn ImageThumbnail(image: ImageResult) -> Element {
    rsx! {
        div {
            class: "image-item",
            
            img {
                src: "{image.thumbnail_url}",
                alt: "{image.title}",
                loading: "lazy",
            }
            
            div {
                class: "image-title",
                title: "{image.title}",
                "{image.title}"
            }
        }
    }
}
