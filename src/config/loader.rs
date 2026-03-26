use crate::SaddleResult;
use exn::{Result, ResultExt, OptionExt};
use std::fs;
use std::path::{Path, PathBuf};

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load() -> SaddleResult<super::Settings> {
        let config_path = Self::find_config_path()?;

        if config_path.exists() {
            Self::load_from_file(&config_path)
        } else {
            let settings = super::Settings::default();
            Self::save_to_file(&settings, &config_path)?;
            Ok(settings)
        }
    }

    pub fn load_from_file(path: impl AsRef<Path>) -> SaddleResult<super::Settings> {
        let content = fs::read_to_string(path.as_ref())
            .or_raise(|| crate::SaddleError::config(format!("Failed to read config from {:?}", path.as_ref())))?;

        let settings: super::Settings = toml::from_str(&content)
            .or_raise(|| crate::SaddleError::parse(format!("Failed to parse config from {:?}", path.as_ref())))?;

        Ok(settings)
    }

    pub fn save_to_file(settings: &super::Settings, path: impl AsRef<Path>) -> SaddleResult<()> {
        let content = toml::to_string_pretty(settings)
            .or_raise(|| crate::SaddleError::parse("Failed to serialize config"))?;

        if let Some(parent) = path.as_ref().parent() {
            fs::create_dir_all(parent)
                .or_raise(|| crate::SaddleError::config(format!("Failed to create config directory: {:?}", parent)))?;
        }

        fs::write(path.as_ref(), content)
            .or_raise(|| crate::SaddleError::config(format!("Failed to write config to {:?}", path.as_ref())))
    }

    fn find_config_path() -> Result<PathBuf, crate::SaddleError> {
        let home = dirs::home_dir()
            .ok_or_raise(|| crate::SaddleError::config("Could not find home directory"))?;

        Ok(home.join(".saddle").join("config.toml"))
    }
}
