use crate::SaddleResult;
use std::path::Path;

#[derive(Debug, Default)]
pub struct PluginRuntime;

impl PluginRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn load_plugin(&self, path: &Path) -> SaddleResult<()> {
        let _ = path;
        Ok(())
    }

    pub fn list_plugins(&self) -> SaddleResult<Vec<String>> {
        Ok(vec![])
    }
}
