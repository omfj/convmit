use clap::Parser;
use colored::*;

use convmit::ai::create_client;
use convmit::cli::Cli;
use convmit::config::Config;
use convmit::git::Git;

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

    if let Some(api_key) = cli.set_gemini_key {
        config.set_gemini_api_key(api_key)?;
        println!("{}", "✓ Gemini API key saved to config".green());
        return Ok(());
    }

    if let Some(model) = cli.set_default_model {
        config.set_default_model(model.clone())?;
        println!(
            "{}",
            format!("✓ Default model set to {} in config", model).green()
        );
        return Ok(());
    }

    let model = cli.model.unwrap_or(config.get_default_model());

    // Validate model configuration
    config.validate_model_config(&model)?;

    // Get API key for the model
    let api_key = config
        .get_api_key_for_model(&model)
        .ok_or(anyhow::anyhow!("No API key found for model {}", model))?;

    let diff = Git::get_staged_diff()?;
    let staged_files = Git::get_staged_files()?;
    if staged_files.is_empty() {
        println!("{}", "ℹ No files staged for commit".yellow());
        return Ok(());
    }

    // Create client using factory pattern
    let client = create_client(model, api_key);
    let commit_message = client.generate_commit_message(&staged_files, &diff).await?;

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
