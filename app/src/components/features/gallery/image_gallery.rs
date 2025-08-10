use dioxus::prelude::*;
use glossia_shared::ImageResult;
use std::collections::HashMap;
use crate::services::ImageFetchState;
use crate::components::features::reading::ImageGallerySkeleton;
use crate::theme::{use_theme, Theme};

#[component]
pub fn ImageGallery(
    word: String,
    image_cache: Signal<HashMap<String, ImageFetchState>>,
) -> Element {
    let theme_mode = use_theme();
    let theme = Theme::from_mode(*theme_mode.read());
    let image_state = image_cache.read().get(&word).cloned();
    
    match image_state {
        Some(ImageFetchState::Loading) | None => rsx! {
            ImageGallerySkeleton {
                theme: theme
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
        }
    }
}
