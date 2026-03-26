use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug)]
pub struct FeatureError {
    message: String,
}

impl FeatureError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for FeatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FeatureError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub struct FeatureManager {
    path: std::path::PathBuf,
}

impl FeatureManager {
    pub fn new() -> Result<Self> {
        let path = Path::new("harness/features.json");
        Ok(Self { path: path.into() })
    }

    pub fn load(&self) -> Result<Vec<Feature>> {
        let content = std::fs::read_to_string(&self.path)
            .context("Failed to read features.json")?;

        let features: Vec<Feature> = serde_json::from_str(&content)
            .context("Failed to parse features.json")?;

        Ok(features)
    }

    pub fn save(&self, features: &[Feature]) -> Result<()> {
        let content = serde_json::to_string_pretty(features)
            .context("Failed to serialize features")?;
        std::fs::write(&self.path, content)
            .context(format!("Failed to write to: {:?}", self.path))?;
        Ok(())
    }

    pub fn get_pending_features(&self) -> Result<Vec<Feature>> {
        let features = self.load()?;
        Ok(features.into_iter().filter(|f| f.status == "pending").collect())
    }

    pub fn mark_completed(&self, feature_id: &str) -> Result<()> {
        let mut features = self.load()?;
        if let Some(feature) = features.iter_mut().find(|f| f.id == feature_id) {
            feature.status = "completed".to_string();
        }
        self.save(&features)
    }
}
