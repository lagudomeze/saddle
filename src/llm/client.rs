use crate::SaddleResult;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai;
use rig::agent::Agent;
use std::sync::Arc;

pub type RigAgent = Agent<openai::responses_api::ResponsesCompletionModel>;

pub struct LlmClient {
    provider: Arc<openai::Client>,
    default_model: String,
}

impl LlmClient {
    pub async fn new(
        api_key: Option<String>,
        _api_base_url: Option<String>,
        default_model: String,
    ) -> SaddleResult<Self> {
        let provider = if let Some(key) = api_key {
            openai::Client::new(&key)
                .map_err(|e| crate::SaddleError::llm(format!("Failed to create client: {}", e)))?
        } else {
            openai::Client::from_env()
        };

        Ok(Self {
            provider: Arc::new(provider),
            default_model,
        })
    }

    pub fn provider(&self) -> &openai::Client {
        &self.provider
    }

    pub fn model(&self) -> &str {
        &self.default_model
    }

    pub async fn complete(&self, prompt_str: &str) -> SaddleResult<String> {
        let agent = self
            .provider
            .agent(&self.default_model)
            .build();

        let response = agent.prompt(prompt_str).await
            .map_err(|e| crate::SaddleError::llm(format!("Completion failed: {}", e)))?;

        Ok(response)
    }

    pub async fn complete_with_system(&self, system: &str, user: &str) -> SaddleResult<String> {
        let prompt = format!("[System: {}]\nUser: {}", system, user);
        self.complete(&prompt).await
    }

    pub fn agent(&self) -> AgentBuilder {
        AgentBuilder {
            provider: self.provider.clone(),
            model: self.default_model.clone(),
            preamble: String::new(),
        }
    }
}

pub struct AgentBuilder {
    provider: Arc<openai::Client>,
    model: String,
    preamble: String,
}

impl AgentBuilder {
    pub fn preamble(mut self, preamble: &str) -> Self {
        self.preamble = preamble.to_string();
        self
    }

    pub fn build(self) -> RigAgent {
        let agent = self.provider.agent(&self.model);
        if self.preamble.is_empty() {
            agent.build()
        } else {
            agent.preamble(&self.preamble).build()
        }
    }
}
