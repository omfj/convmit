mod ai;
mod cli;
mod config;
mod git;

use clap::Parser;
use colored::*;

use crate::ai::{GenerateCommitMessage, Model, claude, openai};
use crate::cli::Cli;
use crate::config::Config;
use crate::git::Git;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = Config::load()?;

    if let Some(api_key) = cli.set_claude_key {
        config.set_claude_api_key(api_key)?;
        println!("{}", "✓ Claude API key saved to config".green());
        return Ok(());
    }

    if let Some(api_key) = cli.set_openai_key {
        config.set_openai_api_key(api_key)?;
        println!("{}", "✓ OpenAI API key saved to config".green());
        return Ok(());
    }

    let model = cli.model.unwrap_or(Model::Haiku3_5);
    let api_key = if model.is_claude() {
        config.get_claude_api_key()
    } else if model.is_openai() {
        config.get_openai_api_key()
    } else {
        None
    }
    .ok_or_else(|| {
        if model.is_claude() {
            "Claude API key not configured. Set with --set-claude-key or CLAUDE_API_KEY env var"
        } else {
            "OpenAI API key not configured. Set with --set-openai-key or OPENAI_API_KEY env var"
        }
    })?;

    let diff = Git::get_staged_diff()?;
    let staged_files = Git::get_staged_files()?;
    if staged_files.is_empty() {
        println!("{}", "ℹ No files staged for commit".yellow());
        return Ok(());
    }

    let commit_message = if model.is_claude() {
        claude::Client::new(api_key, model)
            .generate_commit_message(&staged_files, &diff)
            .await?
    } else if model.is_openai() {
        openai::Client::new(api_key, model)
            .generate_commit_message(&staged_files, &diff)
            .await?
    } else {
        return Err("Unsupported model".into());
    };

    println!(
        "{} {}",
        "Generated commit message:".blue().bold(),
        commit_message.cyan()
    );

    if cli.commit {
        Git::commit(&commit_message)?;
        println!("{}", "✓ Committed with generated message".green().bold());
    }

    Ok(())
}
