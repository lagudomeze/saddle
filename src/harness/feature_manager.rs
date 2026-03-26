use crate::SaddleResult;
use exn::{ResultExt, bail};
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

    pub fn get_feature(&self, feature_id: &str) -> SaddleResult<Option<Feature>> {
        let features = self.load()?;
        Ok(features.into_iter().find(|f| f.id == feature_id))
    }

    pub fn get_completed_features(&self) -> SaddleResult<Vec<Feature>> {
        let features = self.load()?;
        Ok(features.into_iter().filter(|f| f.status == "completed").collect())
    }

    pub fn add_feature(&self, feature: Feature) -> SaddleResult<()> {
        let mut features = self.load()?;
        if features.iter().any(|f| f.id == feature.id) {
            bail!(crate::SaddleError::feature(
                format!("Feature {} already exists", feature.id)
            ));
        }
        features.push(feature);
        self.save(&features)
    }

    pub fn remove_feature(&self, feature_id: &str) -> SaddleResult<()> {
        let mut features = self.load()?;
        let original_len = features.len();
        features.retain(|f| f.id != feature_id);
        if features.len() == original_len {
            bail!(crate::SaddleError::feature(
                format!("Feature {} not found", feature_id)
            ));
        }
        self.save(&features)
    }

    pub fn update_feature(&self, feature_id: &str, updated: Feature) -> SaddleResult<()> {
        let mut features = self.load()?;
        if let Some(feature) = features.iter_mut().find(|f| f.id == feature_id) {
            *feature = updated;
        } else {
            bail!(crate::SaddleError::feature(
                format!("Feature {} not found", feature_id)
            ));
        }
        self.save(&features)
    }

    pub fn mark_completed(&self, feature_id: &str) -> SaddleResult<()> {
        let mut features = self.load()?;
        if let Some(feature) = features.iter_mut().find(|f| f.id == feature_id) {
            feature.status = "completed".to_string();
        }
        self.save(&features)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_manager(temp_dir: &TempDir) -> FeatureManager {
        let path = temp_dir.path().join("features.json");
        let features = vec![
            Feature {
                id: "test-001".to_string(),
                title: "Test Feature 1".to_string(),
                description: "A test feature".to_string(),
                status: "pending".to_string(),
                dependencies: vec![],
                tags: vec!["test".to_string()],
            },
            Feature {
                id: "test-002".to_string(),
                title: "Test Feature 2".to_string(),
                description: "Another test feature".to_string(),
                status: "completed".to_string(),
                dependencies: vec!["test-001".to_string()],
                tags: vec![],
            },
        ];
        let content = serde_json::to_string_pretty(&features).unwrap();
        std::fs::write(&path, content).unwrap();
        FeatureManager { path }
    }

    #[test]
    fn test_load_features() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        let features = manager.load().unwrap();
        assert_eq!(features.len(), 2);
    }

    #[test]
    fn test_get_pending_features() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        let pending = manager.get_pending_features().unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, "test-001");
    }

    #[test]
    fn test_get_completed_features() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        let completed = manager.get_completed_features().unwrap();
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].id, "test-002");
    }

    #[test]
    fn test_get_feature_found() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        let feature = manager.get_feature("test-001").unwrap();
        assert!(feature.is_some());
        assert_eq!(feature.unwrap().title, "Test Feature 1");
    }

    #[test]
    fn test_get_feature_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        let feature = manager.get_feature("nonexistent").unwrap();
        assert!(feature.is_none());
    }

    #[test]
    fn test_add_feature() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        let new_feature = Feature {
            id: "test-003".to_string(),
            title: "New Feature".to_string(),
            description: "A new feature".to_string(),
            status: "pending".to_string(),
            dependencies: vec![],
            tags: vec![],
        };
        
        manager.add_feature(new_feature).unwrap();
        let features = manager.load().unwrap();
        assert_eq!(features.len(), 3);
    }

    #[test]
    fn test_add_duplicate_feature() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        let duplicate = Feature {
            id: "test-001".to_string(),
            title: "Duplicate".to_string(),
            description: "Already exists".to_string(),
            status: "pending".to_string(),
            dependencies: vec![],
            tags: vec![],
        };
        
        let result = manager.add_feature(duplicate);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_feature() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        manager.remove_feature("test-001").unwrap();
        let features = manager.load().unwrap();
        assert_eq!(features.len(), 1);
        assert!(features.iter().all(|f| f.id != "test-001"));
    }

    #[test]
    fn test_remove_nonexistent_feature() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        let result = manager.remove_feature("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_update_feature() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        let updated = Feature {
            id: "test-001".to_string(),
            title: "Updated Title".to_string(),
            description: "Updated description".to_string(),
            status: "completed".to_string(),
            dependencies: vec![],
            tags: vec!["updated".to_string()],
        };
        
        manager.update_feature("test-001", updated).unwrap();
        let feature = manager.get_feature("test-001").unwrap().unwrap();
        assert_eq!(feature.title, "Updated Title");
        assert_eq!(feature.status, "completed");
    }

    #[test]
    fn test_mark_completed() {
        let temp_dir = TempDir::new().unwrap();
        let manager = create_test_manager(&temp_dir);
        
        manager.mark_completed("test-001").unwrap();
        let feature = manager.get_feature("test-001").unwrap().unwrap();
        assert_eq!(feature.status, "completed");
    }
}
