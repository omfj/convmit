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

    #[arg(long, help = "Set the default model in config")]
    pub set_default_model: Option<ai::Model>,

    #[arg(short, long, help = "Specify model to use")]
    pub model: Option<ai::Model>,

    #[arg(
        short,
        long,
        help = "Automatically commit with the generated message",
        default_value = "true"
    )]
    pub commit: bool,
}
