use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use glossia_shared::{AppError, SimplificationRequest, SimplificationResponse, ImageQueryOptimizationRequest, ImageQueryOptimizationResponse};
use std::time::Duration;
use crate::retry_service::RetryService;

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
        Self::new()
    }
}

impl OpenAIClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: "OPENAI_API_KEY_PLACEHOLDER".to_string(),
            base_url: "https://api.openai.com/v1".to_string(),
            retry_service: RetryService::new(3, Duration::from_secs(1)),
        }
    }

    pub async fn simplify(&self, request: SimplificationRequest) -> Result<SimplificationResponse, AppError> {
        let prompt = self.build_prompt(&request.sentence);
        
        let body = json!({
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "response_format": { "type": "json_object" }
        });

        let response = self.retry_service.retry(|| async {
            self.client
                .post(format!("{}/chat/completions", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
        }).await?;

        if response.status().is_success() {
            let openai_response = response.json::<OpenAIResponse>().await?;
            let content = openai_response.choices.get(0)
                .map(|c| c.message.content.clone())
                .ok_or(AppError::InvalidResponseContent)?;
            
            let simplified: SimplificationResponse = serde_json::from_str(&content)?;
            Ok(simplified)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AppError::ApiError(format!("HTTP {}: {}", status, error_text)))
        }
    }
    
    fn build_prompt(&self, sentence: &str) -> String {
        format!(
            r#"
    You are a language assistant helping learners understand complex English.

    Simplify the sentence below using clear and modern English, without losing important meaning.

    Then identify words that are uncommon or difficult for people with only basic (B1) English knowledge. These include:
    - formal or academic words,
    - idioms,
    - phrasal verbs,
    - rare or technical terms.

    For each difficult word, provide a B2+ level meaning in simple English.

    Respond ONLY in this exact JSON format:
    {{
      "original": "{sentence}",
      "simplified": "the simplified version",
      "words": [
        {{ "word": "difficult_word_1", "meaning": "simple explanation" }},
        {{ "word": "difficult_word_2", "meaning": "simple explanation" }}
      ]
    }}

    Sentence to simplify: "{sentence}"
    "#,
            sentence = sentence.replace('"', "\\\"")
        )
    }

    pub async fn get_word_meaning(&self, word: &str, context: &str) -> Result<String, AppError> {
        let prompt = format!(
            r#"Define the word "{}" in simple English using maximum 15 words.

Context: "{}"

Provide a clear, concise definition that helps someone understand the word's meaning in this context.

Respond with ONLY the definition, no extra formatting or quotes."#,
            word, context
        );

        let body = json!({
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "max_tokens": 30
        });

        let response = self.retry_service.retry(|| async {
            self.client
                .post(format!("{}/chat/completions", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .send()
                .await
        }).await?;

        if response.status().is_success() {
            let openai_response = response.json::<OpenAIResponse>().await?;
            let content = openai_response.choices.get(0)
                .map(|c| c.message.content.clone())
                .ok_or(AppError::InvalidResponseContent)?;
            
            Ok(content.trim().to_string())
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AppError::ApiError(format!("HTTP {}: {}", status, error_text)))
        }
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
            "model": "gpt-4o-mini",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.3,
            "response_format": { "type": "json_object" },
            "max_tokens": 50
        });

        let response = self.retry_service.retry_with_max_attempts(2, || async {
            self.client
                .post(format!("{}/chat/completions", self.base_url))
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&body)
                .timeout(Duration::from_secs(10))
                .send()
                .await
        }).await?;

        if response.status().is_success() {
            let openai_response = response.json::<OpenAIResponse>().await?;
            let content = openai_response.choices.get(0)
                .map(|c| c.message.content.clone())
                .ok_or(AppError::InvalidResponseContent)?;
            
            let optimization_response: ImageQueryOptimizationResponse = serde_json::from_str(&content)?;
            Ok(optimization_response)
        } else {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            Err(AppError::ApiError(format!("HTTP {}: {}", status, error_text)))
        }
    }
}
