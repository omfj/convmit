use std::{fmt::Display, str::FromStr};

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

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let model_str = match self {
            Model::Opus4_1 => "claude-opus-4-1-20250805",
            Model::Opus4 => "claude-opus-4-20250514",
            Model::Sonnet4 => "claude-sonnet-4-20250514",
            Model::Sonnet3_7 => "claude-3-7-sonnet-20250219",
            Model::Haiku3_5 => "claude-3-5-haiku-20241022",
            Model::Haiku3 => "claude-3-haiku-20240307",
        };
        write!(f, "{}", model_str)
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
            "Generate a conventional commit message based on the staged files and git diff below.\n\n\
            FORMAT: type(scope): description\n\
            - Use lowercase for type and description\n\
            - Scope is optional but recommended (file/module/feature affected)\n\
            - Description should be 50-72 characters, imperative mood\n\
            - Add '!' after type for breaking changes\n\n\
            COMMIT TYPES:\n\
            - feat: new feature or enhancement\n\
            - fix: bug fix or error correction\n\
            - docs: documentation changes only\n\
            - style: formatting, whitespace (no logic changes)\n\
            - refactor: code restructuring (no feature/bug changes)\n\
            - test: adding or updating tests\n\
            - chore: maintenance, deps, config, build\n\
            - perf: performance improvements\n\
            - ci: CI/CD pipeline changes\n\n\
            SCOPE GUIDELINES:\n\
            - Use filename/module for single file changes\n\
            - Use feature name for multi-file features\n\
            - Use 'readme' for README changes\n\
            - Omit scope for broad changes\n\n\
            EXAMPLES:\n\
            - feat(auth): add OAuth2 login support\n\
            - fix(parser): handle empty input correctly\n\
            - docs(readme): update installation instructions\n\
            - refactor(claude.rs): implement ToString for Model enum\n\
            - style: format code with rustfmt\n\
            - chore(deps): update reqwest to 0.11\n\n\
            INSTRUCTIONS:\n\
            - Analyze the changes to determine the most appropriate type\n\
            - Look for breaking changes (API changes, removed features)\n\
            - Focus on the 'why' not the 'what' in the description\n\
            - Return ONLY the commit message, no explanations\n\n\
            Staged files:\n{}\n\n\
            Git diff:\n{}",
            files.join("\n"),
            diff
        );

        let request = ClaudeRequest {
            model: self.model.to_string(),
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
