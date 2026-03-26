use crate::SaddleResult;
use exn::ResultExt;

pub struct ProgressTracker {
    path: std::path::PathBuf,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            path: std::path::Path::new("harness/progress.md").into(),
        }
    }

    pub fn read(&self) -> SaddleResult<String> {
        std::fs::read_to_string(&self.path)
            .or_raise(|| crate::SaddleError::progress(format!("Failed to read: {:?}", self.path)))
    }

    pub fn update(&self, content: &str) -> SaddleResult<()> {
        std::fs::write(&self.path, content)
            .or_raise(|| crate::SaddleError::progress(format!("Failed to write to: {:?}", self.path)))
    }

    pub fn append(&self, entry: &str) -> SaddleResult<()> {
        let current = self.read().unwrap_or_default();
        let updated = format!("{}\n\n{}", current, entry);
        self.update(&updated)
    }
}
