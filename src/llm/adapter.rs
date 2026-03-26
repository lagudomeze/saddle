use crate::SaddleResult;

pub trait LlmAdapter: Send + Sync {
    fn complete(&self, prompt: &str) -> SaddleResult<String>;
    fn complete_streaming(&self, prompt: &str) -> SaddleResult<()>;
}
