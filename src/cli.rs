use clap::Parser;

use crate::claude;

#[derive(Parser)]
#[command(name = "convmit")]
#[command(about = "Generate conventional commit messages using Claude AI")]
pub struct Cli {
    #[arg(long, help = "Set the Claude API key in config")]
    pub set_api_key: Option<String>,
    #[arg(
        long,
        help = "Specify the Claude model to use (e.g., opus4, sonnet4, haiku3)"
    )]
    pub model: Option<claude::Model>,
    #[arg(short, long, help = "Automatically commit with the generated message", default_value = "true")]
    pub commit: bool,
}
