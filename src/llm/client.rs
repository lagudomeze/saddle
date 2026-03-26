use exn::Result;

#[derive(Debug)]
pub struct LlmError {
    message: String,
}

impl LlmError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for LlmError {}

pub struct LlmClient;

impl LlmClient {
    pub fn new() -> Self {
        Self
    }

    pub fn complete(&self, _prompt: &str) -> Result<String, LlmError> {
        Ok("LLM response placeholder".to_string())
    }
}
