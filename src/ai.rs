use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

mod claude;
mod gemini;
mod mistral;
mod openai;

pub const BASE_PROMPT: &str = r#"Generate a conventional commit message based on the staged files and git diff below.

FORMAT: type(scope): description
- Use lowercase for type and description
- Scope is optional but recommended (file/module/feature affected)
- Description should be 50-72 characters, imperative mood
- Add '!' after type for breaking changes

COMMIT TYPES:
- feat: new feature or enhancement
- fix: bug fix or error correction
- docs: documentation changes only
- style: formatting, whitespace (no logic changes)
- refactor: code restructuring (no feature/bug changes)
- test: adding or updating tests
- chore: maintenance, deps, config, build
- perf: performance improvements
- ci: CI/CD pipeline changes

SCOPE GUIDELINES:
- Use filename/module for single file changes
- Use feature name for multi-file features
- Use 'readme' for README changes
- Omit scope for broad changes

EXAMPLES:
- feat(auth): add OAuth2 login support
- fix(parser): handle empty input correctly
- docs(readme): update installation instructions
- refactor(claude.rs): implement ToString for Model enum
- style: format code with rustfmt
- chore(deps): update reqwest to 0.11

INSTRUCTIONS:
- Analyze the changes to determine the most appropriate type
- Look for breaking changes (API changes, removed features)
- Focus on the 'why' not the 'what' in the description
- Return ONLY the commit message, no explanations
"#;

pub fn build_prompt(files: &[String], diff: &str) -> String {
    format!(
        "{}\n\n Staged files:\n\n {}\n\n Diff:\n\n {}",
        BASE_PROMPT,
        files.join("\n"),
        diff
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
    GPT5,
    GPT5Mini,
    GPT5Nano,
    Gemini2_5Pro,
    Gemini2_5Flash,
    Gemini2_5FlashLite,
    MistralMedium31,
    MagistralMedium11,
    Codestral2508,
    MistralSmall32,
    Ministral8B,
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
            Model::GPT5 => "gpt-5-2025-08-07",
            Model::GPT5Mini => "gpt-5-mini-2025-08-07",
            Model::GPT5Nano => "gpt-5-nano-2025-08-07",
            Model::Gemini2_5Pro => "gemini-2.5-pro",
            Model::Gemini2_5Flash => "gemini-2.5-flash",
            Model::Gemini2_5FlashLite => "gemini-2.5-flash-lite",
            Model::MistralMedium31 => "mistral-medium-2508",
            Model::MagistralMedium11 => "magistral-medium-2507",
            Model::Codestral2508 => "codestral-2508",
            Model::MistralSmall32 => "mistral-small-3.2",
            Model::Ministral8B => "ministral-8b-2410",
        };
        write!(f, "{model_str}")
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
            "gpt-5" => Ok(Model::GPT5),
            "gpt-5-mini" => Ok(Model::GPT5Mini),
            "gpt-5-nano" => Ok(Model::GPT5Nano),
            "gemini-2.5-pro" => Ok(Model::Gemini2_5Pro),
            "gemini-2.5-flash" => Ok(Model::Gemini2_5Flash),
            "gemini-2.5-flash-lite" => Ok(Model::Gemini2_5FlashLite),
            "mistral-medium-31" => Ok(Model::MistralMedium31),
            "magistral-medium-11" => Ok(Model::MagistralMedium11),
            "codestral-2508" => Ok(Model::Codestral2508),
            "mistral-small-32" => Ok(Model::MistralSmall32),
            "ministral-8b" => Ok(Model::Ministral8B),
            _ => Err(anyhow::anyhow!("Unknown model: {}", arg)),
        }
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
            Model::GPT5,
            Model::GPT5Mini,
            Model::GPT5Nano,
            // Gemini models
            Model::Gemini2_5Pro,
            Model::Gemini2_5Flash,
            Model::Gemini2_5FlashLite,
            // Mistral models
            Model::MistralMedium31,
            Model::MagistralMedium11,
            Model::Codestral2508,
            Model::MistralSmall32,
            Model::Ministral8B,
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

    pub fn cli_name(&self) -> &'static str {
        match self {
            Model::Opus4_1 => "opus-4-1",
            Model::Opus4 => "opus-4",
            Model::Sonnet4 => "sonnet-4",
            Model::Sonnet3_7 => "sonnet-3-7",
            Model::Haiku3_5 => "haiku-3-5",
            Model::Haiku3 => "haiku-3",
            Model::GPT5 => "gpt-5",
            Model::GPT5Mini => "gpt-5-mini",
            Model::GPT5Nano => "gpt-5-nano",
            Model::Gemini2_5Pro => "gemini-2.5-pro",
            Model::Gemini2_5Flash => "gemini-2.5-flash",
            Model::Gemini2_5FlashLite => "gemini-2.5-flash-lite",
            Model::MistralMedium31 => "mistral-medium-31",
            Model::MagistralMedium11 => "magistral-medium-11",
            Model::Codestral2508 => "codestral-2508",
            Model::MistralSmall32 => "mistral-small-32",
            Model::Ministral8B => "ministral-8b",
        }
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
        matches!(self, Model::GPT5 | Model::GPT5Mini | Model::GPT5Nano)
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
            Model::MistralMedium31
                | Model::MagistralMedium11
                | Model::Codestral2508
                | Model::MistralSmall32
                | Model::Ministral8B
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
    async fn generate_commit_message(&self, files: &[String], diff: &str)
    -> anyhow::Result<String>;
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

        assert!(!Model::GPT5.is_claude());
        assert!(!Model::GPT5Mini.is_claude());
        assert!(!Model::GPT5Nano.is_claude());
    }

    #[test]
    fn test_model_is_openai() {
        assert!(Model::GPT5.is_openai());
        assert!(Model::GPT5Mini.is_openai());
        assert!(Model::GPT5Nano.is_openai());

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

        let prompt = build_prompt(&files, diff);

        assert!(prompt.contains("src/main.rs"));
        assert!(prompt.contains("src/config.rs"));
        assert!(prompt.contains("diff content here"));
        assert!(prompt.contains("Generate a conventional commit message"));
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
        let model = Model::GPT5;

        let client = create_client(model, api_key);

        // Just verify the client was created successfully
        drop(client);
    }

    #[test]
    fn test_model_is_mistral() {
        assert!(Model::MistralMedium31.is_mistral());
        assert!(Model::MagistralMedium11.is_mistral());
        assert!(Model::Codestral2508.is_mistral());
        assert!(Model::MistralSmall32.is_mistral());
        assert!(Model::Ministral8B.is_mistral());

        assert!(!Model::Opus4_1.is_mistral());
        assert!(!Model::Sonnet4.is_mistral());
        assert!(!Model::GPT5.is_mistral());
        assert!(!Model::Gemini2_5Flash.is_mistral());
    }

    #[test]
    fn test_create_client_with_mistral_model() {
        let api_key = "test-api-key".to_string();
        let model = Model::MistralMedium31;

        let client = create_client(model, api_key);

        // Just verify the client was created successfully
        drop(client);
    }

    #[test]
    fn test_all_models_returns_all_variants() {
        let models = Model::all_models();

        assert_eq!(models.len(), 17);

        assert!(models.iter().any(|m| m.is_claude()));
        assert!(models.iter().any(|m| m.is_openai()));
        assert!(models.iter().any(|m| m.is_gemini()));
        assert!(models.iter().any(|m| m.is_mistral()));

        assert!(models.contains(&Model::Sonnet4));
        assert!(models.contains(&Model::GPT5));
        assert!(models.contains(&Model::Gemini2_5Flash));
        assert!(models.contains(&Model::MistralMedium31));
    }

    #[test]
    fn test_model_provider() {
        assert_eq!(Model::Sonnet4.provider(), "Claude");
        assert_eq!(Model::GPT5.provider(), "OpenAI");
        assert_eq!(Model::Gemini2_5Flash.provider(), "Google Gemini");
        assert_eq!(Model::MistralMedium31.provider(), "Mistral");
    }

    #[test]
    fn test_model_cli_name() {
        assert_eq!(Model::Sonnet4.cli_name(), "sonnet-4");
        assert_eq!(Model::GPT5.cli_name(), "gpt-5");
        assert_eq!(Model::Gemini2_5Flash.cli_name(), "gemini-2.5-flash");
        assert_eq!(Model::MistralMedium31.cli_name(), "mistral-medium-31");
        assert_eq!(Model::Ministral8B.cli_name(), "ministral-8b");
    }

    #[test]
    fn test_cli_name_matches_from_str() {
        for model in Model::all_models() {
            // Test that cli_name can be parsed back to the same model
            assert_eq!(
                Model::from_str(model.cli_name()).unwrap(),
                model,
                "CLI name '{}' for model {:?} should parse back to same model",
                model.cli_name(),
                model
            );
        }
    }
}
