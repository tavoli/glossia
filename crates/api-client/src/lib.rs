use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use glossia_shared::{AppError, SimplificationRequest, SimplificationResponse, ImageSearchRequest, ImageResult, ImageQueryOptimizationRequest, ImageQueryOptimizationResponse};
use std::time::Duration;

// Represents the outer shell of the OpenAI API response
#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

#[derive(Clone)]
pub struct OpenAIClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl Default for OpenAIClient {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenAIClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            // IMPORTANT: Load from config/env in a real app
            api_key: "OPENAI_API_KEY_PLACEHOLDER".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }

    pub async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError> {
        
        let prompt = self.build_prompt(&request.sentence);
        
        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "response_format": { "type": "json_object" }
        });

        // Basic retry logic
        let mut attempts = 0;
        loop {
            
            let response = self.client
                .post(format!("{}/chat/completions", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await;

            match response {
                Ok(res) => {
                    
                    if res.status().is_success() {
                        let openai_response = res.json::<OpenAIResponse>().await?;
                        let content = openai_response.choices.get(0)
                            .map(|c| c.message.content.clone())
                            .ok_or(AppError::InvalidResponseContent)?;
                        
                        
                        let simplified: SimplificationResponse = serde_json::from_str(&content)?;
                        return Ok(simplified);
                    } else {
                        let status = res.status();
                        let error_text = res.text().await.unwrap_or_default();
                        return Err(AppError::ApiError(format!("HTTP {}: {}", status, error_text)));
                    }
                },
                Err(e) => {
                    if attempts >= 3 {
                        return Err(AppError::ApiError(e.to_string()));
                    }
                    attempts += 1;
                    tokio::time::sleep(Duration::from_secs(attempts)).await;
                }
            }
        }
    }
    
    fn build_prompt(&self, sentence: &str) -> String {
        format!(
            r#"Simplify this sentence in modern English without losing detail. 
            Also provide B2-level meanings for difficult words.
            
            Respond ONLY with valid JSON in this exact format:
            {{
                "original": "{}",
                "simplified": "the simplified version",
                "words": [
                    {{"word": "difficult_word", "meaning": "B2-level explanation"}}
                ]
            }}
            
            Sentence to simplify: "{}"
            "#,
            sentence, sentence
        )
    }

    pub async fn optimize_image_query(&self, request: ImageQueryOptimizationRequest) -> Result<ImageQueryOptimizationResponse, AppError> {
        
        let prompt = format!(
    r#"Generate an image search query for the word '{}' based on its contextual meaning.

Context: "{}"
Definition: {}

RULES:
1. If the word itself is already visually descriptive (e.g., "hermit", "lighthouse", "castle"), use it directly
2. Output ONLY valid JSON: {{"optimized_query": "your query"}}
3. Maximum 4 words
4. Add context words that enhance, not distract
5. AVOID extracting unrelated or inappropriate descriptors from context
6. Focus on the PRIMARY subject and its relevant setting

PROHIBITED:
- NO nudity, body parts, or clothing state descriptors (naked, nude, bare, etc.)
- NO sexual or suggestive content
- NO inappropriate physical descriptions

Examples:
- "hermits" + "sea hermits issuing from" → {{"optimized_query": "hermit on sea"}}
- "lighthouse" + "the old lighthouse keeper" → {{"optimized_query": "lighthouse coastal tower"}}
- "crown" + "heavy crown of responsibility" → {{"optimized_query": "royal crown gold"}}
- "bank" + "river bank was muddy" → {{"optimized_query": "river bank shore"}}

Word: '{}'
Context: '{}'
Meaning: {}
"#,
            request.word, 
            request.sentence_context,
            request.word_meaning,
            request.word,
            request.sentence_context,
            request.word_meaning
        );

        
        let body = json!({
            "model": "gpt-4.1-mini",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "response_format": { "type": "json_object" },
            "max_tokens": 50
        });

        // Basic retry logic with shorter timeout for optimization
        let mut attempts = 0;
        loop {
            
            let response = self.client
                .post(format!("{}/chat/completions", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .timeout(Duration::from_secs(10)) // Shorter timeout for optimization
                .send()
                .await;

            match response {
                Ok(res) => {
                    
                    if res.status().is_success() {
                        let openai_response = res.json::<OpenAIResponse>().await?;
                        let content = openai_response.choices.get(0)
                            .map(|c| c.message.content.clone())
                            .ok_or(AppError::InvalidResponseContent)?;
                        
                        
                        let optimization_response: ImageQueryOptimizationResponse = serde_json::from_str(&content)?;
                        return Ok(optimization_response);
                    } else {
                        let status = res.status();
                        let error_text = res.text().await.unwrap_or_default();
                        return Err(AppError::ApiError(format!("HTTP {}: {}", status, error_text)));
                    }
                },
                Err(e) => {
                    if attempts >= 2 { // Only 2 retries for optimization
                        return Err(AppError::ApiError(format!("Optimization failed: {}", e)));
                    }
                    attempts += 1;
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BraveImageResponse {
    #[serde(rename = "type")]
    response_type: String,
    results: Vec<BraveImageItem>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BraveImageItem {
    #[serde(rename = "type")]
    item_type: String,
    title: Option<String>,
    url: Option<String>,
    source: Option<String>,
    page_fetched: Option<String>,
    thumbnail: Option<BraveThumbnail>,
    properties: Option<BraveImageProperties>,
    meta_url: Option<BraveMetaUrl>,
    confidence: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BraveThumbnail {
    src: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct BraveImageProperties {
    url: String,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct BraveMetaUrl {
    scheme: Option<String>,
    netloc: Option<String>,
    hostname: Option<String>,
    favicon: Option<String>,
    path: Option<String>,
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
            return Err(AppError::ApiError(format!("HTTP {}: {}", status, error_text)));
        }

        let brave_response: BraveImageResponse = response.json().await?;
        

        let images: Vec<ImageResult> = brave_response.results
            .into_iter()
            .filter_map(|item| {
                // Only process items that have both thumbnail and properties
                if let (Some(_thumbnail), Some(properties)) = (&item.thumbnail, &item.properties) {
                    // Filter by dimensions if available
                    if let (Some(width), Some(height)) = (properties.width, properties.height) {
                        if width >= 275 && height >= 275 {
                            Some(item)
                        } else {
                            None
                        }
                    } else {
                        Some(item) // Include if dimensions not available
                    }
                } else {
                    None
                }
            })
            .take(request.count as usize)
            .map(|item| ImageResult {
                url: item.properties.as_ref().unwrap().url.clone(),
                thumbnail_url: item.thumbnail.as_ref().unwrap().src.clone().unwrap_or_default(),
                title: item.title.unwrap_or_default(),
                width: item.properties.as_ref().unwrap().width,
                height: item.properties.as_ref().unwrap().height,
            })
            .collect();

        Ok(images)
    }
}
