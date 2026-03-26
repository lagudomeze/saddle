use serde::{Deserialize, Serialize};
use std::path::Path;
use anyhow::{Context, Result};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features {
    pub features: Vec<Feature>,
}

pub struct FeatureManager {
    path: std::path::PathBuf,
}

impl FeatureManager {
    pub fn new() -> Result<Self> {
        let path = Path::new("harness/features.json");
        Ok(Self { path: path.into() })
    }

    pub fn load(&self) -> Result<Features> {
        let content = std::fs::read_to_string(&self.path)
            .context("Failed to read features.json")?;
        serde_json::from_str(&content).context("Failed to parse features.json")
    }

    pub fn save(&self, features: &Features) -> Result<()> {
        let content = serde_json::to_string_pretty(features)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    pub fn get_pending_features(&self) -> Result<Vec<Feature>> {
        let features = self.load()?;
        Ok(features.features.into_iter().filter(|f| f.status == "pending").collect())
    }

    pub fn mark_completed(&self, feature_id: &str) -> Result<()> {
        let mut features = self.load()?;
        if let Some(feature) = features.features.iter_mut().find(|f| f.id == feature_id) {
            feature.status = "completed".to_string();
        }
        self.save(&features)
    }
}
