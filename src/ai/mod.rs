use std::{fmt::Display, str::FromStr};

pub mod claude;
pub mod openai;

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

#[derive(Debug, Clone, clap::ValueEnum)]
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
            "gpt-5" => Ok(Model::GPT5),
            "gpt-5-mini" => Ok(Model::GPT5Mini),
            "gpt-5-nano" => Ok(Model::GPT5Nano),
            _ => Err(anyhow::anyhow!("Unknown model: {}", arg)),
        }
    }
}

impl Model {
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
}

pub trait GenerateCommitMessage {
    async fn generate_commit_message(&self, files: &[String], diff: &str)
    -> anyhow::Result<String>;
}
