use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug)]
pub struct ProgressError {
    message: String,
}

impl ProgressError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ProgressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ProgressError {}

pub struct ProgressTracker {
    path: std::path::PathBuf,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            path: Path::new("harness/progress.md").into(),
        }
    }

    pub fn read(&self) -> Result<String> {
        std::fs::read_to_string(&self.path)
            .context(format!("Failed to read: {:?}", self.path))
    }

    pub fn update(&self, content: &str) -> Result<()> {
        std::fs::write(&self.path, content)
            .context(format!("Failed to write to: {:?}", self.path))?;
        Ok(())
    }

    pub fn append(&self, entry: &str) -> Result<()> {
        let current = self.read().unwrap_or_default();
        let updated = format!("{}\n\n{}", current, entry);
        self.update(&updated)
    }
}
