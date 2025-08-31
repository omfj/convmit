use std::str::FromStr;

use crate::models::{ClaudeRequest, ClaudeResponse, ErrorResponse, Message};
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum Model {
    Opus4_1,
    Opus4,
    Sonnet4,
    Sonnet3_7,
    Haiku3_5,
    Haiku3,
}

impl Model {
    pub fn as_str(&self) -> &'static str {
        match self {
            Model::Opus4_1 => "claude-opus-4-1-20250805",
            Model::Opus4 => "claude-opus-4-20250514",
            Model::Sonnet4 => "claude-sonnet-4-20250514",
            Model::Sonnet3_7 => "claude-3-7-sonnet-20250219",
            Model::Haiku3_5 => "claude-3-5-haiku-20241022",
            Model::Haiku3 => "claude-3-haiku-20240307",
        }
    }
}

impl FromStr for Model {
    type Err = anyhow::Error;

    fn from_str(arg: &str) -> anyhow::Result<Self> {
        match arg {
            "opus-4-1" => Ok(Model::Opus4_1),
            "opus-4" => Ok(Model::Opus4),
            "sonnet-4" => Ok(Model::Sonnet4),
            "sonnet-3-7" => Ok(Model::Sonnet3_7),
            "haiku-3-5" => Ok(Model::Haiku3_5),
            "haiku-3" => Ok(Model::Haiku3),
            _ => Err(anyhow::anyhow!("Unknown model: {}", arg)),
        }
    }
}

pub struct ClaudeClient {
    api_key: String,
    model: Model,
    client: reqwest::Client,
}

impl ClaudeClient {
    pub fn new(api_key: String, model: Model) -> Self {
        Self {
            api_key,
            model,
            client: reqwest::Client::new(),
        }
    }

    pub async fn generate_commit_message(&self, files: &[String], diff: &str) -> Result<String> {
        let prompt = format!(
            "Based on the following staged files and git diff, generate a conventional commit message. \
            Follow the format: type(scope): description\n\
            Common types: feat, fix, docs, style, refactor, test, chore\n\
            Keep the description concise and under 50 characters.\n\
            Only return the commit message, nothing else.\n\n\
            Staged files:\n{}\n\n\
            Git diff:\n{}",
            files.join("\n"),
            diff
        );

        let request = ClaudeRequest {
            model: self.model.as_str().to_string(),
            max_tokens: 1024,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .client
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
                return Err(anyhow::anyhow!("Claude API error: {}", error_response.error.message));
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
