use crate::ai::{self, GenerateCommitMessage, Model, build_prompt};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct MistralRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct MistralResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
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
    model: ai::Model,
}

impl Client {
    pub fn new(api_key: String, model: Model) -> Self {
        assert!(model.is_mistral(), "Model must be a Mistral model");

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
        let prompt = build_prompt(files, diff);

        let request = MistralRequest {
            model: self.model.to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
            max_tokens: 1024,
            temperature: 0.3,
        };

        let response = http_client
            .post("https://api.mistral.ai/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&error_text) {
                return Err(anyhow::anyhow!(
                    "Mistral API error: {}",
                    error_response.error.message
                ));
            } else {
                return Err(anyhow::anyhow!("HTTP error {}: {}", status, error_text));
            }
        }

        let mistral_response: MistralResponse = response.json().await?;

        if let Some(choice) = mistral_response.choices.first() {
            Ok(choice.message.content.trim().to_string())
        } else {
            Err(anyhow::anyhow!("No response from Mistral"))
        }
    }
}