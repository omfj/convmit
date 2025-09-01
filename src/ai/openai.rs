use reqwest::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use crate::ai::{GenerateCommitMessage, Model, build_prompt};

#[derive(Serialize)]
pub struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
pub struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
pub struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Deserialize)]
struct ErrorDetail {
    message: String,
}

pub struct Client {
    api_key: String,
    model: Model,
}

impl Client {
    pub fn new(api_key: String, model: Model) -> Self {
        assert!(model.is_openai(), "Model must be an OpenAI model");

        Self { api_key, model }
    }
}

impl GenerateCommitMessage for Client {
    async fn generate_commit_message(
        &self,
        files: &[String],
        diff: &str,
    ) -> anyhow::Result<String> {
        let http_client = reqwest::Client::new();
        let prompt = build_prompt(files, diff);

        let request = OpenAIRequest {
            model: self.model.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = http_client
            .post("https://api.openai.com/v1/chat/completions")
            .header("content-type", "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
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

        let openai_response: OpenAIResponse = response.json().await?;

        if let Some(choice) = openai_response.choices.first() {
            Ok(choice.message.content.trim().to_string())
        } else {
            Err(anyhow::anyhow!("No response from Claude"))
        }
    }
}
