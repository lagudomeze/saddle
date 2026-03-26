use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug)]
pub struct HandoffError {
    message: String,
}

impl HandoffError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for HandoffError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HandoffError {}

pub struct HandoffGenerator {
    path: std::path::PathBuf,
}

impl HandoffGenerator {
    pub fn new() -> Self {
        Self {
            path: Path::new("harness/handoff.md").into(),
        }
    }

    pub fn generate(&self, completed: &[String], next_steps: &[String], decisions: &[String]) -> Result<()> {
        let content = format!(
            "# 交接报告\n\n## 已完成工作\n{}\n\n## 下一步计划\n{}\n\n## 技术决策\n{}\n",
            Self::format_list(completed),
            Self::format_list(next_steps),
            Self::format_list(decisions)
        );
        std::fs::write(&self.path, content)
            .context(format!("Failed to write to: {:?}", self.path))?;
        Ok(())
    }

    fn format_list(items: &[String]) -> String {
        items.iter().map(|s| format!("- {}", s)).collect::<Vec<_>>().join("\n")
    }
}
