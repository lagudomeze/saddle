use anyhow::Result;

pub trait LlmAdapter: Send + Sync {
    fn complete(&self, prompt: &str) -> Result<String>;
    fn complete_streaming(&self, prompt: &str) -> Result<()>;
}
