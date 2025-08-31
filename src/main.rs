mod claude;
mod cli;
mod config;
mod git;
mod models;

use clap::Parser;
use colored::*;

use crate::claude::ClaudeClient;
use crate::cli::Cli;
use crate::config::Config;
use crate::git::Git;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    if let Some(api_key) = cli.set_api_key {
        config.set_api_key(api_key)?;
        println!("{}", "✓ API key saved to config".green());
        return Ok(());
    }

    let api_key = config
        .get_api_key()
        .ok_or("API key not configured. Get one at https://console.anthropic.com/settings/keys then use --set-api-key to set it")?;

    let staged_files = Git::get_staged_files()?;
    if staged_files.is_empty() {
        println!("{}", "ℹ No files staged for commit".yellow());
        return Ok(());
    }

    let diff = Git::get_staged_diff()?;
    let model = cli.model.unwrap_or(claude::Model::Haiku3_5);
    let claude_client = ClaudeClient::new(api_key, model);
    let commit_message = claude_client
        .generate_commit_message(&staged_files, &diff)
        .await?;

    println!("{} {}", "Generated commit message:".blue().bold(), commit_message.cyan());
    
    if cli.commit {
        Git::commit(&commit_message)?;
        println!("{}", "✓ Committed with generated message".green().bold());
    }
    
    Ok(())
}
