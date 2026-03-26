use anyhow::Result;
use std::path::Path;

pub struct PluginRuntime;

impl PluginRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn load_plugin(&self, path: &Path) -> Result<()> {
        let _ = path;
        Ok(())
    }

    pub fn list_plugins(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }
}
