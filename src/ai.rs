use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

mod claude;
mod gemini;
mod mistral;
mod openai;

pub const SYSTEM_PROMPT: &str = r#"<task>Generate a conventional commit message from staged files and git diff.</task>

<format>
  type(scope): description
  - type and description in lowercase
  - scope optional
  - description 50 to 72 characters, imperative mood
  - add '!' after type for breaking changes
</format>

<types>
  feat, fix, docs, style, refactor, test, chore, perf, ci
</types>

<scope_guidelines>
  - filename/module for single file
  - feature name for multi-file feature
  - 'readme' for README
  - omit for broad changes
</scope_guidelines>

<examples>
  feat(auth): add OAuth2 login support
  fix(parser): handle empty input correctly
  docs(readme): update installation instructions
  refactor(claude.rs): implement ToString for Model enum
  style: format code with rustfmt
  chore(deps): update reqwest to 0.11
</examples>

<instructions>
  - choose correct type
  - detect breaking changes
  - focus on intent, not implementation
  - output only the commit message
</instructions>
"#;

pub fn build_user_prompt(files: &[String], diff: &str, additional_context: Option<&str>) -> String {
    let context_section = if let Some(ctx) = additional_context {
        format!(
            r#"

    <additional_context>
{}
    </additional_context>"#,
            ctx
        )
    } else {
        String::new()
    };

    format!(
        r#"
  <context>
    <staged_files>
{}
    </staged_files>

    <diff>
{}
    </diff>{}
  </context>"#,
        files.join("\n"),
        diff,
        context_section
    )
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, clap::ValueEnum)]
pub enum Model {
    Opus4_1,
    Opus4,
    Sonnet4,
    Sonnet3_7,
    Haiku3_5,
    Haiku3,
    Gpt5,
    Gpt5Mini,
    Gpt5Nano,
    Gemini2_5Pro,
    Gemini2_5Flash,
    Gemini2_5FlashLite,
    MistralMedium3_1,
    MagistralMedium1_1,
    MistralSmall3_2,
    Ministral8b,
}

impl Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let possible_values = <Self as clap::ValueEnum>::to_possible_value(self);

        if let Some(possible_value) = possible_values {
            write!(f, "{}", possible_value.get_name())
        } else {
            write!(f, "{:?}", self)
        }
    }
}

impl FromStr for Model {
    type Err = anyhow::Error;

    fn from_str(arg: &str) -> anyhow::Result<Self> {
        // Use clap's ValueEnum parsing
        <Self as clap::ValueEnum>::from_str(arg, true)
            .map_err(|_| anyhow::anyhow!("Unknown model: {}", arg))
    }
}

impl Model {
    pub fn all_models() -> Vec<Model> {
        vec![
            // Claude models
            Model::Opus4_1,
            Model::Opus4,
            Model::Sonnet4,
            Model::Sonnet3_7,
            Model::Haiku3_5,
            Model::Haiku3,
            // OpenAI models
            Model::Gpt5,
            Model::Gpt5Mini,
            Model::Gpt5Nano,
            // Gemini models
            Model::Gemini2_5Pro,
            Model::Gemini2_5Flash,
            Model::Gemini2_5FlashLite,
            // Mistral models
            Model::MistralMedium3_1,
            Model::MagistralMedium1_1,
            Model::MistralSmall3_2,
            Model::Ministral8b,
        ]
    }

    pub fn provider(&self) -> &'static str {
        if self.is_claude() {
            "Claude"
        } else if self.is_openai() {
            "OpenAI"
        } else if self.is_gemini() {
            "Google Gemini"
        } else if self.is_mistral() {
            "Mistral"
        } else {
            "Unknown"
        }
    }

    pub fn to_api_str(&self) -> String {
        let str = match self {
            Model::Opus4_1 => "claude-opus-4-1-20250805",
            Model::Opus4 => "claude-opus-4-20250514",
            Model::Sonnet4 => "claude-sonnet-4-20250514",
            Model::Sonnet3_7 => "claude-3-7-sonnet-20250219",
            Model::Haiku3_5 => "claude-3-5-haiku-20241022",
            Model::Haiku3 => "claude-3-haiku-20240307",
            Model::Gpt5 => "gpt-5-2025-08-07",
            Model::Gpt5Mini => "gpt-5-mini-2025-08-07",
            Model::Gpt5Nano => "gpt-5-nano-2025-08-07",
            Model::Gemini2_5Pro => "gemini-2.5-pro",
            Model::Gemini2_5Flash => "gemini-2.5-flash",
            Model::Gemini2_5FlashLite => "gemini-2.5-flash-lite",
            Model::MistralMedium3_1 => "mistral-medium-2508",
            Model::MagistralMedium1_1 => "magistral-medium-2507",
            Model::MistralSmall3_2 => "mistral-small-3.2",
            Model::Ministral8b => "ministral-8b-2410",
        };

        str.to_string()
    }

    pub fn is_claude(&self) -> bool {
        matches!(
            self,
            Model::Opus4_1
                | Model::Opus4
                | Model::Sonnet4
                | Model::Sonnet3_7
                | Model::Haiku3_5
                | Model::Haiku3
        )
    }

    pub fn is_openai(&self) -> bool {
        matches!(self, Model::Gpt5 | Model::Gpt5Mini | Model::Gpt5Nano)
    }

    pub fn is_gemini(&self) -> bool {
        matches!(
            self,
            Model::Gemini2_5Pro | Model::Gemini2_5Flash | Model::Gemini2_5FlashLite
        )
    }

    pub fn is_mistral(&self) -> bool {
        matches!(
            self,
            Model::MistralMedium3_1
                | Model::MagistralMedium1_1
                | Model::MistralSmall3_2
                | Model::Ministral8b
        )
    }
}

pub fn create_client(model: Model, api_key: String) -> Box<dyn GenerateCommitMessage> {
    if model.is_claude() {
        Box::new(claude::Client::new(api_key, model))
    } else if model.is_openai() {
        Box::new(openai::Client::new(api_key, model))
    } else if model.is_gemini() {
        Box::new(gemini::Client::new(api_key, model))
    } else if model.is_mistral() {
        Box::new(mistral::Client::new(api_key, model))
    } else {
        panic!("Unsupported model: {model:?}")
    }
}

#[async_trait::async_trait]
pub trait GenerateCommitMessage {
    async fn generate_commit_message(
        &self,
        files: &[String],
        diff: &str,
        context: Option<&str>,
    ) -> anyhow::Result<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_is_claude() {
        assert!(Model::Opus4_1.is_claude());
        assert!(Model::Opus4.is_claude());
        assert!(Model::Sonnet4.is_claude());
        assert!(Model::Sonnet3_7.is_claude());
        assert!(Model::Haiku3_5.is_claude());
        assert!(Model::Haiku3.is_claude());

        assert!(!Model::Gpt5.is_claude());
        assert!(!Model::Gpt5Mini.is_claude());
        assert!(!Model::Gpt5Nano.is_claude());
    }

    #[test]
    fn test_model_is_openai() {
        assert!(Model::Gpt5.is_openai());
        assert!(Model::Gpt5Mini.is_openai());
        assert!(Model::Gpt5Nano.is_openai());

        assert!(!Model::Opus4_1.is_openai());
        assert!(!Model::Opus4.is_openai());
        assert!(!Model::Sonnet4.is_openai());
        assert!(!Model::Sonnet3_7.is_openai());
        assert!(!Model::Haiku3_5.is_openai());
        assert!(!Model::Haiku3.is_openai());
    }
    #[test]
    fn test_build_prompt() {
        let files = vec!["src/main.rs".to_string(), "src/config.rs".to_string()];
        let diff = "diff content here";

        let prompt = build_user_prompt(&files, diff, None);

        assert!(prompt.contains("src/main.rs"));
        assert!(prompt.contains("src/config.rs"));
        assert!(prompt.contains("diff content here"));
    }

    #[test]
    fn test_create_client_with_claude_model() {
        let api_key = "test-api-key".to_string();
        let model = Model::Sonnet4;

        let client = create_client(model, api_key);

        // Just verify the client was created successfully
        drop(client);
    }

    #[test]
    fn test_create_client_with_openai_model() {
        let api_key = "test-api-key".to_string();
        let model = Model::Gpt5;

        let client = create_client(model, api_key);

        // Just verify the client was created successfully
        drop(client);
    }

    #[test]
    fn test_model_is_mistral() {
        assert!(Model::MistralMedium3_1.is_mistral());
        assert!(Model::MagistralMedium1_1.is_mistral());
        assert!(Model::MistralSmall3_2.is_mistral());
        assert!(Model::Ministral8b.is_mistral());

        assert!(!Model::Opus4_1.is_mistral());
        assert!(!Model::Sonnet4.is_mistral());
        assert!(!Model::Gpt5.is_mistral());
        assert!(!Model::Gemini2_5Flash.is_mistral());
    }

    #[test]
    fn test_create_client_with_mistral_model() {
        let api_key = "test-api-key".to_string();
        let model = Model::MistralMedium3_1;

        let client = create_client(model, api_key);

        // Just verify the client was created successfully
        drop(client);
    }

    #[test]
    fn test_all_models_returns_all_variants() {
        let models = Model::all_models();

        assert_eq!(models.len(), 16);

        assert!(models.iter().any(|m| m.is_claude()));
        assert!(models.iter().any(|m| m.is_openai()));
        assert!(models.iter().any(|m| m.is_gemini()));
        assert!(models.iter().any(|m| m.is_mistral()));

        assert!(models.contains(&Model::Sonnet4));
        assert!(models.contains(&Model::Gpt5));
        assert!(models.contains(&Model::Gemini2_5Flash));
        assert!(models.contains(&Model::MistralMedium3_1));
    }

    #[test]
    fn test_model_provider() {
        assert_eq!(Model::Sonnet4.provider(), "Claude");
        assert_eq!(Model::Gpt5.provider(), "OpenAI");
        assert_eq!(Model::Gemini2_5Flash.provider(), "Google Gemini");
        assert_eq!(Model::MistralMedium3_1.provider(), "Mistral");
    }
}
