use exn::Result;
use std::path::Path;

#[derive(Debug)]
pub struct PluginError {
    message: String,
}

impl PluginError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for PluginError {}

pub struct PluginRuntime;

impl PluginRuntime {
    pub fn new() -> Self {
        Self
    }

    pub fn load_plugin(&self, path: &Path) -> Result<(), PluginError> {
        let _ = path;
        Ok(())
    }

    pub fn list_plugins(&self) -> Result<Vec<String>, PluginError> {
        Ok(vec![])
    }
}
