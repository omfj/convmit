use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub claude_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    pub gemini_api_key: Option<String>,
    pub mistral_api_key: Option<String>,
    pub default_model: Option<crate::ai::Model>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            claude_api_key: None,
            openai_api_key: None,
            gemini_api_key: None,
            mistral_api_key: None,
            default_model: Some(crate::ai::Model::Haiku4_5),
        }
    }
}

impl Config {
    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
            .join("convmit");

        fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn get_claude_api_key(&self) -> Option<String> {
        self.claude_api_key
            .clone()
            .or(std::env::var("CLAUDE_API_KEY").ok())
    }

    pub fn set_claude_api_key(&mut self, key: String) -> Result<()> {
        self.claude_api_key = Some(key);
        self.save()
    }

    pub fn get_openai_api_key(&self) -> Option<String> {
        self.openai_api_key
            .clone()
            .or(std::env::var("OPENAI_API_KEY").ok())
    }

    pub fn set_openai_api_key(&mut self, key: String) -> Result<()> {
        self.openai_api_key = Some(key);
        self.save()
    }

    pub fn get_gemini_api_key(&self) -> Option<String> {
        self.gemini_api_key
            .clone()
            .or(std::env::var("GEMINI_API_KEY").ok())
    }

    pub fn set_gemini_api_key(&mut self, key: String) -> Result<()> {
        self.gemini_api_key = Some(key);
        self.save()
    }

    pub fn validate_model_config(&self, model: &crate::ai::Model) -> Result<()> {
        match model {
            m if m.is_claude() && self.get_claude_api_key().is_none() => Err(anyhow::anyhow!(
                "Claude API key required for {}. Set with --set-claude-key or CLAUDE_API_KEY env var",
                model
            )),
            m if m.is_openai() && self.get_openai_api_key().is_none() => Err(anyhow::anyhow!(
                "OpenAI API key required for {}. Set with --set-openai-key or OPENAI_API_KEY env var",
                model
            )),
            m if m.is_gemini() && self.get_gemini_api_key().is_none() => Err(anyhow::anyhow!(
                "Gemini API key required for {}. Set with --set-gemini-key or GEMINI_API_KEY env var",
                model
            )),
            m if m.is_mistral() && self.get_mistral_api_key().is_none() => Err(anyhow::anyhow!(
                "Mistral API key required for {}. Set with --set-mistral-key or MISTRAL_API_KEY env var",
                model
            )),
            _ => Ok(()),
        }
    }

    pub fn get_mistral_api_key(&self) -> Option<String> {
        self.mistral_api_key
            .clone()
            .or(std::env::var("MISTRAL_API_KEY").ok())
    }

    pub fn set_mistral_api_key(&mut self, key: String) -> Result<()> {
        self.mistral_api_key = Some(key);
        self.save()
    }

    pub fn get_api_key_for_model(&self, model: &crate::ai::Model) -> Option<String> {
        if model.is_claude() {
            self.get_claude_api_key()
        } else if model.is_openai() {
            self.get_openai_api_key()
        } else if model.is_gemini() {
            self.get_gemini_api_key()
        } else if model.is_mistral() {
            self.get_mistral_api_key()
        } else {
            None
        }
    }

    pub fn get_default_model(&self) -> crate::ai::Model {
        self.default_model
            .clone()
            .unwrap_or(crate::ai::Model::Haiku4_5)
    }

    pub fn set_default_model(&mut self, model: crate::ai::Model) -> Result<()> {
        self.default_model = Some(model);
        self.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::Model;

    fn create_test_config() -> Config {
        Config {
            claude_api_key: Some("test-claude-key".to_string()),
            openai_api_key: Some("test-openai-key".to_string()),
            gemini_api_key: Some("test-gemini-key".to_string()),
            mistral_api_key: Some("test-mistral-key".to_string()),
            default_model: Some(Model::Sonnet4),
        }
    }

    fn create_empty_config() -> Config {
        Config {
            claude_api_key: None,
            openai_api_key: None,
            gemini_api_key: None,
            mistral_api_key: None,
            default_model: None,
        }
    }

    #[test]
    fn test_get_claude_api_key_from_config() {
        let config = create_test_config();
        assert_eq!(
            config.get_claude_api_key(),
            Some("test-claude-key".to_string())
        );
    }

    #[test]
    fn test_get_openai_api_key_from_config() {
        let config = create_test_config();
        assert_eq!(
            config.get_openai_api_key(),
            Some("test-openai-key".to_string())
        );
    }

    #[test]
    fn test_get_claude_api_key_from_env() {
        let config = create_empty_config();

        // Set environment variable
        unsafe {
            std::env::set_var("CLAUDE_API_KEY", "env-claude-key");
        }

        assert_eq!(
            config.get_claude_api_key(),
            Some("env-claude-key".to_string())
        );

        // Clean up
        unsafe {
            std::env::remove_var("CLAUDE_API_KEY");
        }
    }

    #[test]
    fn test_get_openai_api_key_from_env() {
        let config = create_empty_config();

        // Set environment variable
        unsafe {
            std::env::set_var("OPENAI_API_KEY", "env-openai-key");
        }

        assert_eq!(
            config.get_openai_api_key(),
            Some("env-openai-key".to_string())
        );

        // Clean up
        unsafe {
            std::env::remove_var("OPENAI_API_KEY");
        }
    }

    #[test]
    fn test_config_takes_precedence_over_env() {
        let config = create_test_config();

        // Set environment variables
        unsafe {
            std::env::set_var("CLAUDE_API_KEY", "env-claude-key");
            std::env::set_var("OPENAI_API_KEY", "env-openai-key");
        }

        // Config values should take precedence
        assert_eq!(
            config.get_claude_api_key(),
            Some("test-claude-key".to_string())
        );
        assert_eq!(
            config.get_openai_api_key(),
            Some("test-openai-key".to_string())
        );

        // Clean up
        unsafe {
            std::env::remove_var("CLAUDE_API_KEY");
            std::env::remove_var("OPENAI_API_KEY");
        }
    }

    #[test]
    fn test_validate_model_config_success() {
        let config = create_test_config();

        // Should succeed for Claude models when Claude key is present
        assert!(config.validate_model_config(&Model::Sonnet4).is_ok());
        assert!(config.validate_model_config(&Model::Haiku4_5).is_ok());

        // Should succeed for OpenAI models when OpenAI key is present
        assert!(config.validate_model_config(&Model::Gpt5).is_ok());
        assert!(config.validate_model_config(&Model::Gpt5Mini).is_ok());
    }

    #[test]
    fn test_validate_model_config_failure() {
        let config = create_empty_config();

        // Should fail for Claude models when Claude key is missing
        let result = config.validate_model_config(&Model::Sonnet4);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Claude API key required")
        );

        // Should fail for OpenAI models when OpenAI key is missing
        let result = config.validate_model_config(&Model::Gpt5);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("OpenAI API key required")
        );
    }

    #[test]
    fn test_get_api_key_for_model() {
        let config = create_test_config();

        // Test Claude models
        assert_eq!(
            config.get_api_key_for_model(&Model::Sonnet4),
            Some("test-claude-key".to_string())
        );
        assert_eq!(
            config.get_api_key_for_model(&Model::Haiku4_5),
            Some("test-claude-key".to_string())
        );

        // Test OpenAI models
        assert_eq!(
            config.get_api_key_for_model(&Model::Gpt5),
            Some("test-openai-key".to_string())
        );
        assert_eq!(
            config.get_api_key_for_model(&Model::Gpt5Mini),
            Some("test-openai-key".to_string())
        );
    }

    #[test]
    fn test_get_api_key_for_model_empty_config() {
        // Clean up any lingering env vars first
        unsafe {
            std::env::remove_var("CLAUDE_API_KEY");
            std::env::remove_var("OPENAI_API_KEY");
        }

        let config = create_empty_config();

        // Should return None when no keys are configured
        assert_eq!(config.get_api_key_for_model(&Model::Sonnet4), None);
        assert_eq!(config.get_api_key_for_model(&Model::Gpt5), None);
    }

    #[test]
    fn test_partial_config() {
        let claude_only_config = Config {
            claude_api_key: Some("claude-key".to_string()),
            openai_api_key: None,
            gemini_api_key: None,
            mistral_api_key: None,
            default_model: None,
        };

        let openai_only_config = Config {
            claude_api_key: None,
            openai_api_key: Some("openai-key".to_string()),
            gemini_api_key: None,
            mistral_api_key: None,
            default_model: None,
        };

        let gemini_only_config = Config {
            claude_api_key: None,
            openai_api_key: None,
            gemini_api_key: Some("gemini-key".to_string()),
            mistral_api_key: None,
            default_model: None,
        };

        let mistral_only_config = Config {
            claude_api_key: None,
            openai_api_key: None,
            gemini_api_key: None,
            mistral_api_key: Some("mistral-key".to_string()),
            default_model: None,
        };

        // Claude-only config
        assert!(
            claude_only_config
                .validate_model_config(&Model::Sonnet4)
                .is_ok()
        );
        assert!(
            claude_only_config
                .validate_model_config(&Model::Gpt5)
                .is_err()
        );

        // OpenAI-only config
        assert!(
            openai_only_config
                .validate_model_config(&Model::Sonnet4)
                .is_err()
        );
        assert!(
            openai_only_config
                .validate_model_config(&Model::Gpt5)
                .is_ok()
        );

        // Gemini-only config
        assert!(
            gemini_only_config
                .validate_model_config(&Model::Sonnet4)
                .is_err()
        );
        assert!(
            gemini_only_config
                .validate_model_config(&Model::Gpt5)
                .is_err()
        );
        assert!(
            gemini_only_config
                .validate_model_config(&Model::Gemini2_5Flash)
                .is_ok()
        );

        // Mistral-only config
        assert!(
            mistral_only_config
                .validate_model_config(&Model::Sonnet4)
                .is_err()
        );
        assert!(
            mistral_only_config
                .validate_model_config(&Model::Gpt5)
                .is_err()
        );
        assert!(
            mistral_only_config
                .validate_model_config(&Model::MistralMedium3_1)
                .is_ok()
        );
    }

    #[test]
    fn test_get_default_model() {
        let config = create_test_config();
        assert_eq!(config.get_default_model(), Model::Sonnet4);
    }

    #[test]
    fn test_get_default_model_fallback() {
        let config = create_empty_config();
        assert_eq!(config.get_default_model(), Model::Haiku4_5);
    }

    #[test]
    fn test_get_default_model_with_default() {
        let config = Config::default();
        assert_eq!(config.get_default_model(), Model::Haiku4_5);
    }

    #[test]
    fn test_get_mistral_api_key_from_config() {
        let config = create_test_config();
        assert_eq!(
            config.get_mistral_api_key(),
            Some("test-mistral-key".to_string())
        );
    }

    #[test]
    fn test_get_mistral_api_key_from_env() {
        let config = create_empty_config();

        // Set environment variable
        unsafe {
            std::env::set_var("MISTRAL_API_KEY", "env-mistral-key");
        }

        assert_eq!(
            config.get_mistral_api_key(),
            Some("env-mistral-key".to_string())
        );

        // Clean up
        unsafe {
            std::env::remove_var("MISTRAL_API_KEY");
        }
    }

    #[test]
    fn test_validate_mistral_model_config() {
        let config = create_test_config();
        assert!(
            config
                .validate_model_config(&Model::MistralMedium3_1)
                .is_ok()
        );

        let empty_config = create_empty_config();
        let result = empty_config.validate_model_config(&Model::MistralMedium3_1);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Mistral API key required")
        );
    }
}
