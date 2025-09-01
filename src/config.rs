use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub claude_api_key: Option<String>,
    pub openai_api_key: Option<String>,
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
}
