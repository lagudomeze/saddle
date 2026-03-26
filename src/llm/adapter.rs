use exn::Result;

#[derive(Debug)]
pub struct AdapterError {
    message: String,
}

impl AdapterError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AdapterError {}

pub trait LlmAdapter: Send + Sync {
    fn complete(&self, prompt: &str) -> Result<String, AdapterError>;
    fn complete_streaming(&self, prompt: &str) -> Result<(), AdapterError>;
}
