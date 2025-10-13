use clap::Parser;

use crate::ai;

#[derive(Parser)]
#[command(name = "convmit")]
#[command(about = "Generate conventional commit messages using Claude AI")]
pub struct Cli {
    #[arg(long, help = "Set the Claude API key in config")]
    pub set_claude_key: Option<String>,

    #[arg(long, help = "Set the OpenAI API key in config")]
    pub set_openai_key: Option<String>,

    #[arg(long, help = "Set the Gemini API key in config")]
    pub set_gemini_key: Option<String>,

    #[arg(long, help = "Set the Mistral API key in config")]
    pub set_mistral_key: Option<String>,

    #[arg(long, help = "Set the default model in config")]
    pub set_default_model: Option<ai::Model>,

    #[arg(long, help = "List all available models")]
    pub list_models: bool,

    #[arg(short, long, help = "Specify model to use")]
    pub model: Option<ai::Model>,

    #[arg(
        short,
        long,
        help = "Automatically commit with the generated message",
        default_value = "false"
    )]
    pub no_commit: bool,

    #[arg(long, help = "Edit the generated message before using it")]
    pub edit: bool,

    #[arg(
        long = "exclude",
        value_name = "FILE",
        num_args = 1..,
        value_delimiter = ',',
        help = "Files to exclude from the generated prompt"
    )]
    pub exclude: Vec<String>,

    #[arg(
        long = "only",
        value_name = "FILE",
        num_args = 1..,
        value_delimiter = ',',
        help = "Limit the prompt to the specified files"
    )]
    pub only: Vec<String>,
}
