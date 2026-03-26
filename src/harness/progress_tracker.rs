use std::path::Path;
use anyhow::{Context, Result};

pub struct ProgressTracker {
    path: std::path::PathBuf,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self { path: Path::new("harness/progress.md").into() }
    }

    pub fn read(&self) -> Result<String> {
        std::fs::read_to_string(&self.path).context("Failed to read progress.md")
    }

    pub fn update(&self, content: &str) -> Result<()> {
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    pub fn append(&self, entry: &str) -> Result<()> {
        let current = self.read().unwrap_or_default();
        let updated = format!("{}\n\n{}", current, entry);
        self.update(&updated)
    }
}
