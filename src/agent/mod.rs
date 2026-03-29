use rig::agent::Agent;
use rig::completion::Prompt;
use rig::providers::openai::responses_api::ResponsesCompletionModel;

pub type RigAgent = Agent<ResponsesCompletionModel>;

pub struct AgentExecutor {
    agent: RigAgent,
}

impl AgentExecutor {
    pub fn new(agent: RigAgent) -> Self {
        Self { agent }
    }

    pub async fn prompt(&self, user_input: &str) -> crate::SaddleResult<String> {
        Ok(self
            .agent
            .prompt(user_input)
            .await
            .map_err(|e| crate::SaddleError::llm(format!("Agent prompt failed: {}", e)))?)
    }
}

pub mod presets {
    pub fn assistant_system() -> &'static str {
        "You are a helpful, precise, and concise AI assistant. \
         Provide accurate information and acknowledge when you're uncertain. \
         Format responses clearly with appropriate structure."
    }

    pub fn code_assistant_system() -> &'static str {
        "You are an expert software developer. Write clean, efficient, and well-documented code. \
         Follow best practices and language-specific conventions. \
         Explain your reasoning when helpful."
    }

    pub fn researcher_system() -> &'static str {
        "You are a thorough research assistant. Analyze topics comprehensively, \
         cite sources when possible, and distinguish facts from opinions. \
         Present multiple perspectives when relevant."
    }

    pub fn critic_system() -> &'static str {
        "You are a critical thinker who evaluates arguments and ideas rigorously. \
         Identify strengths and weaknesses, spot logical fallacies, \
         and provide constructive feedback."
    }
}

pub use rig::completion::request::ToolDefinition;
