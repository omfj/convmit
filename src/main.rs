use std::collections::{BTreeMap, HashSet};

use clap::Parser;
use colored::*;

use convmit::ai::{Model, create_client};
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

    if let Some(api_key) = cli.set_mistral_key {
        config.set_mistral_api_key(api_key)?;
        println!("{}", "✓ Mistral API key saved to config".green());
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

    if cli.list_models {
        println!("{}", "Available models:".blue().bold());

        let models_by_provider: BTreeMap<&str, Vec<_>> =
            Model::all_models()
                .into_iter()
                .fold(BTreeMap::new(), |mut acc, model| {
                    acc.entry(model.provider()).or_default().push(model);
                    acc
                });

        for (provider, models) in models_by_provider {
            println!("\n{}", provider.cyan().bold());
            for model in models {
                println!(
                    "  {} ({})",
                    model.to_string().white(),
                    model.to_api_str().dimmed()
                );
            }
        }
        return Ok(());
    }

    let model = cli.model.unwrap_or(config.get_default_model());

    // Validate model configuration
    config.validate_model_config(&model)?;

    // Get API key for the model
    let api_key = config
        .get_api_key_for_model(&model)
        .ok_or(anyhow::anyhow!("No API key found for model {}", model))?;

    let staged_files = Git::get_staged_files()?;
    if staged_files.is_empty() {
        println!("{}", "ℹ No files staged for commit".yellow());
        return Ok(());
    }

    let filtered_files = apply_file_filters(staged_files, &cli.only, &cli.exclude);
    if filtered_files.is_empty() {
        println!(
            "{}",
            "ℹ No staged files matched the provided filters".yellow()
        );
        return Ok(());
    }

    let diff = Git::get_staged_diff(&filtered_files)?;

    // Create client using factory pattern
    let client = create_client(model, api_key);
    let commit_message = client
        .generate_commit_message(&filtered_files, &diff)
        .await?;

    println!("{}", commit_message);

    if !cli.no_commit {
        Git::commit(&commit_message)?;
        println!("{}", "✓ Committed with generated message".green().bold());
    }

    Ok(())
}

fn apply_file_filters(
    staged_files: Vec<String>,
    only: &[String],
    exclude: &[String],
) -> Vec<String> {
    let only_set: HashSet<&str> = only.iter().map(|s| s.as_str()).collect();
    let exclude_set: HashSet<&str> = exclude.iter().map(|s| s.as_str()).collect();

    staged_files
        .into_iter()
        .filter(|file| {
            (only_set.is_empty() || only_set.contains(file.as_str()))
                && !exclude_set.contains(file.as_str())
        })
        .collect()
}
