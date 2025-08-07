use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use glossia_shared::{AppError, SimplificationRequest, SimplificationResponse, ImageQueryOptimizationRequest, ImageQueryOptimizationResponse};
use std::time::Duration;
use crate::retry_service::RetryService;
use crate::traits::LLMClient;
use async_trait::async_trait;

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
    retry_service: RetryService,
}

impl Default for OpenAIClient {
    fn default() -> Self {
        Self::new().expect("Failed to create OpenAI client")
    }
}

impl OpenAIClient {
    pub fn new() -> Result<Self, AppError> {
        // Load environment variables
        dotenvy::dotenv().ok();
        
        let api_key = std::env::var("OPENAI_API_KEY")
            .map_err(|_| AppError::config_error("OPENAI_API_KEY environment variable must be set"))?;
        let base_url = std::env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
            
        Ok(Self {
            client: Client::new(),
            api_key,
            base_url,
            retry_service: RetryService::new(3, Duration::from_secs(1)),
        })
    }
    
    pub fn with_config(api_key: String, base_url: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            retry_service: RetryService::new(3, Duration::from_secs(1)),
        }
    }

    pub async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError> {
        let prompt = self.build_simplification_prompt(&request.sentence);
        
        let body = json!({
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "response_format": { "type": "json_object" }
        });

        let content = self.make_chat_request(&body).await?;
        let simplified: SimplificationResponse = serde_json::from_str(&content)?;
        Ok(simplified)
    }
    
    fn build_simplification_prompt(&self, sentence: &str) -> String {
        format!(
            r#"
    You are a language assistant helping learners understand complex English.

    Simplify the sentence below using clear and modern English, without losing important meaning.

    Then identify words AND phrases that are uncommon or difficult for people with only basic (B1) English knowledge. These include:
    - formal or academic words,
    - idioms,
    - phrasal verbs,
    - multi-word expressions,
    - collocations,
    - rare or technical terms.

    For each difficult word or phrase, provide a B2+ level meaning in simple English.

    Respond ONLY in this exact JSON format:
    {{
      "original": "{sentence}",
      "simplified": "the simplified version",
      "words": [
        {{ "word": "difficult_word_1", "meaning": "simple explanation", "is_phrase": false }},
        {{ "word": "multi word phrase", "meaning": "simple explanation", "is_phrase": true }},
        {{ "word": "another_word", "meaning": "simple explanation", "is_phrase": false }}
      ]
    }}

    Sentence to simplify: "{sentence}"
    "#,
            sentence = sentence.replace('"', "\\\"")
        )
    }

    fn build_word_meaning_prompt(&self, word: &str, context: &str) -> String {
        format!(
            r#"Define the word "{}" in simple English using maximum 15 words.

Context: "{}"

Provide a clear, concise definition that helps someone understand the word's meaning in this context.

Respond with ONLY the definition, no extra formatting or quotes."#,
            word, context
        )
    }

    fn build_image_optimization_prompt(&self, request: &ImageQueryOptimizationRequest) -> String {
        format!(
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
        )
    }

    pub async fn get_word_meaning(&self, word: &str, context: &str) -> Result<String, AppError> {
        let prompt = self.build_word_meaning_prompt(word, context);

        let body = json!({
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "max_tokens": 30
        });

        let content = self.make_chat_request(&body).await?;
        Ok(content.trim().to_string())
    }

    pub async fn optimize_image_query(&self, request: ImageQueryOptimizationRequest) -> Result<ImageQueryOptimizationResponse, AppError> {
        let prompt = self.build_image_optimization_prompt(&request);

        let body = json!({
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "response_format": { "type": "json_object" },
            "max_tokens": 50
        });

        let content = self.make_chat_request_with_options(&body, Some(2), Some(Duration::from_secs(10))).await?;
        let optimization_response: ImageQueryOptimizationResponse = serde_json::from_str(&content)?;
        Ok(optimization_response)
    }

    // Shared HTTP request logic
    async fn make_chat_request(&self, body: &serde_json::Value) -> Result<String, AppError> {
        self.make_chat_request_with_options(body, None, None).await
    }

    async fn make_chat_request_with_options(
        &self, 
        body: &serde_json::Value, 
        max_attempts: Option<u32>, 
        timeout: Option<Duration>
    ) -> Result<String, AppError> {
        let response = self.make_request_with_options("/chat/completions", body, max_attempts, timeout).await?;
        self.parse_chat_response(response).await
    }

    async fn make_request(&self, endpoint: &str, body: &serde_json::Value) -> Result<reqwest::Response, AppError> {
        self.make_request_with_options(endpoint, body, None, None).await
    }

    async fn make_request_with_options(
        &self, 
        endpoint: &str, 
        body: &serde_json::Value, 
        max_attempts: Option<u32>, 
        timeout: Option<Duration>
    ) -> Result<reqwest::Response, AppError> {
        let retry_func = || async {
            let mut request = self.client
                .post(format!("{}{}", self.base_url, endpoint))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(body);
            
            if let Some(timeout_duration) = timeout {
                request = request.timeout(timeout_duration);
            }
            
            request.send().await
        };

        match max_attempts {
            Some(attempts) => self.retry_service.retry_with_max_attempts(attempts, retry_func).await,
            None => self.retry_service.retry(retry_func).await,
        }
    }

    async fn parse_chat_response(&self, response: reqwest::Response) -> Result<String, AppError> {
        let status = response.status();
        
        if status.is_success() {
            let openai_response = response.json::<OpenAIResponse>().await?;
            let content = openai_response.choices.get(0)
                .map(|c| c.message.content.clone())
                .ok_or(AppError::InvalidResponseContent)?;
            Ok(content)
        } else {
            let error_text = response.text().await.unwrap_or_default();
            Err(AppError::http_error(status.as_u16(), error_text))
        }
    }
}

#[async_trait]
impl LLMClient for OpenAIClient {
    async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError> {
        self.simplify(request).await
    }
    
    async fn get_word_meaning(&self, word: &str, context: &str) -> Result<String, AppError> {
        self.get_word_meaning(word, context).await
    }
    
    async fn optimize_image_query(&self, request: ImageQueryOptimizationRequest) -> Result<ImageQueryOptimizationResponse, AppError> {
        self.optimize_image_query(request).await
    }
}
