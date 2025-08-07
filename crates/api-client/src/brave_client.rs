use reqwest::Client;
use serde::Deserialize;
use glossia_shared::{AppError, ImageSearchRequest, ImageResult};

#[derive(Debug, Deserialize)]
struct BraveImageResponse {
    results: Vec<BraveImageItem>,
}

#[derive(Debug, Deserialize)]
struct BraveImageItem {
    title: Option<String>,
    thumbnail: Option<BraveThumbnail>,
    properties: Option<BraveImageProperties>,
}

#[derive(Debug, Deserialize)]
struct BraveThumbnail {
    src: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BraveImageProperties {
    url: String,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Clone)]
pub struct BraveImageClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl Default for BraveImageClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BraveImageClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: "BSAQLLRY6kYB1YNZuFL_s9BajQQaZDB".to_string(),
            base_url: "https://api.search.brave.com/res/v1".to_string(),
        }
    }

    pub async fn search_images(&self, request: ImageSearchRequest) -> Result<Vec<ImageResult>, AppError> {
        let url = format!(
            "{}/images/search?q={}&count={}&safesearch=strict&search_lang=en&country=us",
            self.base_url,
            urlencoding::encode(&request.query),
            request.count
        );

        let response = self.client
            .get(&url)
            .header("Accept", "application/json")
            .header("X-Subscription-Token", &self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::http_error(status.as_u16(), error_text));
        }

        let brave_response: BraveImageResponse = response.json().await?;

        let images: Vec<ImageResult> = brave_response.results
            .into_iter()
            .filter_map(|item| {
                if let (Some(_thumbnail), Some(properties)) = (&item.thumbnail, &item.properties) {
                    if let (Some(width), Some(height)) = (properties.width, properties.height) {
                        if width >= 275 && height >= 275 {
                            Some(item)
                        } else {
                            None
                        }
                    } else {
                        Some(item)
                    }
                } else {
                    None
                }
            })
            .take(request.count as usize)
            .filter_map(|item| {
                if let (Some(properties), Some(thumbnail)) = (&item.properties, &item.thumbnail) {
                    Some(ImageResult {
                        url: properties.url.clone(),
                        thumbnail_url: thumbnail.src.clone().unwrap_or_default(),
                        title: item.title.unwrap_or_default(),
                        width: properties.width,
                        height: properties.height,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(images)
    }
}
