use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ConfigError {
    message: String,
}

impl ConfigError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ConfigError {}

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load() -> Result<super::Settings> {
        let config_path = Self::find_config_path()?;

        if config_path.exists() {
            Self::load_from_file(&config_path)
        } else {
            let settings = super::Settings::default();
            Self::save_to_file(&settings, &config_path)?;
            Ok(settings)
        }
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> Result<super::Settings> {
        let content = fs::read_to_string(path.as_ref())
            .context(format!("Failed to read config from {:?}", path.as_ref()))?;

        let settings: super::Settings = toml::from_str(&content)
            .context("Failed to parse config file")?;

        Ok(settings)
    }

    pub fn save_to_file(settings: &super::Settings, path: impl AsRef<Path>) -> Result<()> {
        let content = toml::to_string_pretty(settings)
            .context("Failed to serialize config")?;

        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create config directory: {:?}", parent))?;
        }

        fs::write(path.as_ref(), content)
            .context(format!("Failed to write config to {:?}", path.as_ref()))?;

        Ok(())
    }

    fn find_config_path() -> Result<PathBuf> {
        let home = dirs::home_dir()
            .context("Could not find home directory")?;

        Ok(home.join(".saddle").join("config.toml"))
    }
}
