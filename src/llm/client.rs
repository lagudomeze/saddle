use anyhow::Result;

pub struct LlmClient;

impl LlmClient {
    pub fn new() -> Self {
        Self
    }

    pub fn complete(&self, _prompt: &str) -> Result<String> {
        Ok("LLM response placeholder".to_string())
    }
}
