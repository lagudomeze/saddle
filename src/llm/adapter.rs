use crate::SaddleResult;

pub trait LlmAdapter: Send + Sync {
    fn complete(&self, prompt: &str) -> impl std::future::Future<Output = SaddleResult<String>> + Send;
    fn complete_with_system(&self, system: &str, user: &str) -> impl std::future::Future<Output = SaddleResult<String>> + Send;
}
