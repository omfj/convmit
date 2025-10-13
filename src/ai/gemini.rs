use serde::{Deserialize, Serialize};

use crate::ai::{GenerateCommitMessage, Model, SYSTEM_PROMPT, build_user_prompt};

#[derive(Serialize)]
struct GeminiRequest {
    system_instruction: Vec<Content>,
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize)]
struct PartResponse {
    text: String,
}

pub struct Client {
    api_key: String,
    model: Model,
}

impl Client {
    pub fn new(api_key: String, model: Model) -> Self {
        assert!(model.is_gemini(), "Model must be a Gemini model");

        Self { api_key, model }
    }
}

// TODO: Add system instruction
// https://ai.google.dev/gemini-api/docs/text-generation

#[async_trait::async_trait]
impl GenerateCommitMessage for Client {
    async fn generate_commit_message(
        &self,
        files: &[String],
        diff: &str,
    ) -> anyhow::Result<String> {
        let http_client = reqwest::Client::new();
        let prompt = build_user_prompt(files, diff);

        let request = GeminiRequest {
            system_instruction: vec![Content {
                parts: vec![Part {
                    text: SYSTEM_PROMPT.to_string(),
                }],
            }],
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let response = http_client
            .post(format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent",
                self.model.to_api_str()
            ))
            .header("content-type", "application/json")
            .header("x-goog-api-key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("HTTP error {}: {}", status, error_text));
        }

        let gemini_response: GeminiResponse = response.json().await?;

        if let Some(content) = gemini_response
            .candidates
            .first()
            .ok_or(anyhow::anyhow!("No candidates in Gemini response"))?
            .content
            .parts
            .first()
        {
            Ok(content.text.trim().to_string())
        } else {
            Err(anyhow::anyhow!("No response from Claude"))
        }
    }
}
