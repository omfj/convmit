use crate::ai::{self, GenerateCommitMessage, Model, SYSTEM_PROMPT, build_user_prompt};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<Content>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Content {
    text: String,
    #[serde(rename = "type")]
    content_type: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    #[serde(rename = "type")]
    error: ErrorDetail,
}

#[derive(Deserialize)]
struct ErrorDetail {
    #[serde(rename = "type")]
    message: String,
}

pub struct Client {
    api_key: String,
    model: ai::Model,
}

impl Client {
    pub fn new(api_key: String, model: Model) -> Self {
        assert!(model.is_claude(), "Model must be a Claude model");

        Self { api_key, model }
    }
}

#[async_trait::async_trait]
impl GenerateCommitMessage for Client {
    async fn generate_commit_message(
        &self,
        files: &[String],
        diff: &str,
    ) -> anyhow::Result<String> {
        let http_client = reqwest::Client::new();
        let prompt = build_user_prompt(files, diff);

        let request = ClaudeRequest {
            model: self.model.to_api_str(),
            max_tokens: 1024,
            system: SYSTEM_PROMPT.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = http_client
            .post("https://api.anthropic.com/v1/messages")
            .header("content-type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .header("x-api-key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&error_text) {
                return Err(anyhow::anyhow!(
                    "Claude API error: {}",
                    error_response.error.message
                ));
            } else {
                return Err(anyhow::anyhow!("HTTP error {}: {}", status, error_text));
            }
        }

        let claude_response: ClaudeResponse = response.json().await?;

        if let Some(content) = claude_response.content.first() {
            Ok(content.text.trim().to_string())
        } else {
            Err(anyhow::anyhow!("No response from Claude"))
        }
    }
}
