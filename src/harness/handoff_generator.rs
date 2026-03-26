use crate::SaddleResult;
use exn::{Result, ResultExt};
use std::path::Path;

#[derive(Debug, Default)]
pub struct HandoffGenerator {
    path: std::path::PathBuf,
}

impl HandoffGenerator {
    pub fn new() -> Self {
        Self {
            path: Path::new("harness/handoff.md").into(),
        }
    }

    pub fn generate(&self, completed: &[String], next_steps: &[String], decisions: &[String]) -> SaddleResult<()> {
        let content = format!(
            "# 交接报告\n\n## 已完成工作\n{}\n\n## 下一步计划\n{}\n\n## 技术决策\n{}\n",
            Self::format_list(completed),
            Self::format_list(next_steps),
            Self::format_list(decisions)
        );
        std::fs::write(&self.path, content)
            .or_raise(|| crate::SaddleError::Handoff(format!("Failed to write to: {:?}", self.path)))?;
        Ok(())
    }

    fn format_list(items: &[String]) -> String {
        items.iter().map(|s| format!("- {}", s)).collect::<Vec<_>>().join("\n")
    }
}
