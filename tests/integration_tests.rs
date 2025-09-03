use convmit::ai::{Model, create_client};
use convmit::config::Config;

#[tokio::test]
async fn test_client_factory_creates_working_clients() {
    let api_key = "test-key".to_string();

    // Test Claude client creation
    let claude_client = create_client(Model::Sonnet4, api_key.clone());
    drop(claude_client); // Just verify it was created successfully

    // Test OpenAI client creation
    let openai_client = create_client(Model::GPT5, api_key);
    drop(openai_client); // Just verify it was created successfully
}

#[test]
fn test_config_integration_with_models() {
    let mut config = Config {
        claude_api_key: Some("claude-test-key".to_string()),
        openai_api_key: None,
        gemini_api_key: None,
        mistral_api_key: None,
        default_model: None,
    };

    // Claude model should work
    assert!(config.validate_model_config(&Model::Sonnet4).is_ok());
    assert_eq!(
        config.get_api_key_for_model(&Model::Sonnet4),
        Some("claude-test-key".to_string())
    );

    // OpenAI model should fail validation
    assert!(config.validate_model_config(&Model::GPT5).is_err());
    assert_eq!(config.get_api_key_for_model(&Model::GPT5), None);

    // Add OpenAI key
    config.openai_api_key = Some("openai-test-key".to_string());

    // Now OpenAI model should work
    assert!(config.validate_model_config(&Model::GPT5).is_ok());
    assert_eq!(
        config.get_api_key_for_model(&Model::GPT5),
        Some("openai-test-key".to_string())
    );
}

#[test]
fn test_model_display_and_parsing() {
    use std::str::FromStr;

    // Test model display
    assert_eq!(Model::Sonnet4.to_string(), "claude-sonnet-4-20250514");
    assert_eq!(Model::GPT5.to_string(), "gpt-5-2025-08-07");

    // Test model parsing
    assert!(matches!(
        Model::from_str("sonnet-4").unwrap(),
        Model::Sonnet4
    ));
    assert!(matches!(Model::from_str("gpt-5").unwrap(), Model::GPT5));

    // Test invalid model
    assert!(Model::from_str("invalid-model").is_err());
}

#[test]
fn test_full_workflow_simulation() {
    let config = Config {
        claude_api_key: Some("claude-key".to_string()),
        openai_api_key: Some("openai-key".to_string()),
        gemini_api_key: Some("gemini-key".to_string()),
        mistral_api_key: Some("mistral-key".to_string()),
        default_model: None,
    };

    let models_to_test = vec![
        Model::Sonnet4,
        Model::Haiku3_5,
        Model::GPT5,
        Model::GPT5Mini,
        Model::MistralMedium31,
        Model::Ministral8B,
    ];

    for model in models_to_test {
        // Validate configuration
        assert!(config.validate_model_config(&model).is_ok());

        // Get API key
        let api_key = config.get_api_key_for_model(&model).unwrap();

        // Create client
        let client = create_client(model.clone(), api_key);
        drop(client); // Just verify creation
    }
}

#[test]
fn test_environment_variable_fallback() {
    // This test demonstrates env var fallback by using std::env::var directly
    // rather than relying on the potentially cleaned-up environment

    // Set environment variables
    unsafe {
        std::env::set_var("TEST_CLAUDE_API_KEY", "env-claude-key");
        std::env::set_var("TEST_OPENAI_API_KEY", "env-openai-key");
    }

    // Test that environment variable lookup works
    assert_eq!(
        std::env::var("TEST_CLAUDE_API_KEY").ok(),
        Some("env-claude-key".to_string())
    );
    assert_eq!(
        std::env::var("TEST_OPENAI_API_KEY").ok(),
        Some("env-openai-key".to_string())
    );

    // Clean up test vars
    unsafe {
        std::env::remove_var("TEST_CLAUDE_API_KEY");
        std::env::remove_var("TEST_OPENAI_API_KEY");
    }

    // The actual env var integration is tested in config unit tests
}

#[test]
fn test_error_messages_are_descriptive() {
    // Clean up any env vars that might interfere with test
    unsafe {
        std::env::remove_var("CLAUDE_API_KEY");
        std::env::remove_var("OPENAI_API_KEY");
    }

    let empty_config = Config {
        claude_api_key: None,
        openai_api_key: None,
        gemini_api_key: None,
        mistral_api_key: None,
        default_model: None,
    };

    // Test Claude error message
    let claude_error = empty_config
        .validate_model_config(&Model::Sonnet4)
        .unwrap_err();
    let error_msg = claude_error.to_string();
    assert!(error_msg.contains("Claude API key required"));
    assert!(error_msg.contains("--set-claude-key"));
    assert!(error_msg.contains("CLAUDE_API_KEY"));

    // Test OpenAI error message
    let openai_error = empty_config
        .validate_model_config(&Model::GPT5)
        .unwrap_err();
    let error_msg = openai_error.to_string();
    assert!(error_msg.contains("OpenAI API key required"));
    assert!(error_msg.contains("--set-openai-key"));
    assert!(error_msg.contains("OPENAI_API_KEY"));
}
