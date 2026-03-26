use crate::SaddleResult;
use exn::{Result, ResultExt};
use serde::{Deserialize, Serialize};
use std::path::Path;

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
    pub fn new() -> Result<Self, crate::SaddleError> {
        let path = Path::new("harness/features.json");
        Ok(Self { path: path.into() })
    }

    pub fn load(&self) -> SaddleResult<Vec<Feature>> {
        let content = std::fs::read_to_string(&self.path)
            .map_err(crate::SaddleError::io)?;

        let features: Vec<Feature> = serde_json::from_str(&content)
            .or_raise(|| crate::SaddleError::parse("Failed to parse features.json"))?;

        Ok(features)
    }

    pub fn save(&self, features: &[Feature]) -> SaddleResult<()> {
        let content = serde_json::to_string_pretty(features)
            .or_raise(|| crate::SaddleError::parse("Failed to serialize features"))?;
        std::fs::write(&self.path, content)
            .or_raise(|| crate::SaddleError::feature(format!("Failed to write to: {:?}", self.path)))
    }

    pub fn get_pending_features(&self) -> SaddleResult<Vec<Feature>> {
        let features = self.load()?;
        Ok(features.into_iter().filter(|f| f.status == "pending").collect())
    }

    pub fn mark_completed(&self, feature_id: &str) -> SaddleResult<()> {
        let mut features = self.load()?;
        if let Some(feature) = features.iter_mut().find(|f| f.id == feature_id) {
            feature.status = "completed".to_string();
        }
        self.save(&features)
    }
}
