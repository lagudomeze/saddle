//! 多模型 Provider 抽象层
//!
//! 支持多种 LLM Provider：OpenAI、Claude、Gemini、DeepSeek、Ollama 等

use crate::{SaddleError, SaddleResult};
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::openai;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 支持的模型提供商
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    OpenAI,
    Anthropic,
    DeepSeek,
    Gemini,
    Ollama,
    Mistral,
    #[serde(other)]
    Unknown,
}

impl ProviderType {
    /// 根据模型名称推断 Provider
    pub fn from_model(model: &str) -> Self {
        let model_lower = model.to_lowercase();
        if model_lower.contains("claude") || model_lower.contains("anthropic") {
            ProviderType::Anthropic
        } else if model_lower.contains("gemini") || model_lower.contains("google") {
            ProviderType::Gemini
        } else if model_lower.contains("deepseek") {
            ProviderType::DeepSeek
        } else if model_lower.contains("ollama") {
            ProviderType::Ollama
        } else if model_lower.contains("mistral") || model_lower.contains("mixtral") {
            ProviderType::Mistral
        } else {
            // 默认 OpenAI 兼容
            ProviderType::OpenAI
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderType::OpenAI => "openai",
            ProviderType::Anthropic => "anthropic",
            ProviderType::DeepSeek => "deepseek",
            ProviderType::Gemini => "gemini",
            ProviderType::Ollama => "ollama",
            ProviderType::Mistral => "mistral",
            ProviderType::Unknown => "unknown",
        }
    }
}

/// 统一错误转换
fn map_rig_error(e: impl std::fmt::Display) -> SaddleError {
    SaddleError::llm(format!("Provider error: {}", e))
}

/// 模型配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// 模型标识符（如 "gpt-4", "claude-3-opus", "deepseek-chat"）
    pub model: String,
    /// API Base URL（可选，用于自定义端点）
    pub api_base_url: Option<String>,
    /// API Key 环境变量名
    pub api_key_env: Option<String>,
}

impl ModelConfig {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
            api_base_url: None,
            api_key_env: None,
        }
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.api_base_url = Some(url.into());
        self
    }

    pub fn with_api_key_env(mut self, env: impl Into<String>) -> Self {
        self.api_key_env = Some(env.into());
        self
    }
}

/// Token 使用量统计
#[derive(Debug, Clone, Default)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// 统一错误转换
fn client_error(e: impl std::fmt::Display) -> SaddleError {
    SaddleError::llm(format!("OpenAI client error: {}", e))
}

/// OpenAI Provider 实现
pub struct OpenAiProvider {
    client: Arc<openai::Client>,
    model: String,
}

impl OpenAiProvider {
    pub async fn new(config: &ModelConfig) -> SaddleResult<Self> {
        let client = if let Some(ref key_env) = config.api_key_env {
            let api_key = std::env::var(key_env)
                .map_err(|_| SaddleError::llm(format!("Environment variable {} not found", key_env)))?;
            openai::Client::new(&api_key)
                .map_err(client_error)?
        } else {
            openai::Client::from_env()
        };

        Ok(Self {
            client: Arc::new(client),
            model: config.model.clone(),
        })
    }

    pub async fn complete(&self, prompt: &str) -> SaddleResult<String> {
        let agent = self.client.agent(&self.model).build();
        let response = agent.prompt(prompt).await.map_err(|e| map_rig_error(e))?;
        Ok(response)
    }

    pub async fn complete_with_system(&self, system: &str, user: &str) -> SaddleResult<String> {
        let prompt = format!("[System: {}]\nUser: {}", system, user);
        self.complete(&prompt).await
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn provider_type(&self) -> ProviderType {
        ProviderType::OpenAI
    }
}

/// Provider 工厂
pub struct ProviderFactory;

impl ProviderFactory {
    /// 检测模型所属的 Provider 类型
    pub fn detect_provider(model: &str) -> ProviderType {
        ProviderType::from_model(model)
    }

    /// 获取已知模型列表
    pub fn known_models(provider: ProviderType) -> Vec<&'static str> {
        match provider {
            ProviderType::OpenAI => vec![
                "gpt-4",
                "gpt-4-turbo",
                "gpt-4o",
                "gpt-4o-mini",
                "gpt-3.5-turbo",
            ],
            ProviderType::Anthropic => vec![
                "claude-3-5-sonnet-latest",
                "claude-3-5-haiku-latest",
                "claude-3-opus-latest",
                "claude-3-sonnet-latest",
                "claude-3-haiku-latest",
            ],
            ProviderType::DeepSeek => vec![
                "deepseek-chat",
                "deepseek-coder",
            ],
            ProviderType::Gemini => vec![
                "gemini-1.5-pro",
                "gemini-1.5-flash",
                "gemini-1.0-pro",
            ],
            ProviderType::Ollama => vec![
                "llama3",
                "llama3.1",
                "mistral",
                "mixtral",
                "codellama",
            ],
            ProviderType::Mistral => vec![
                "mistral-large",
                "mistral-medium",
                "mistral-small",
                "mixtral-8x7b",
            ],
            ProviderType::Unknown => vec![],
        }
    }

    /// 列出所有支持的模型
    pub fn all_known_models() -> Vec<(&'static str, ProviderType)> {
        Self::known_models(ProviderType::OpenAI)
            .into_iter()
            .map(|m| (m, ProviderType::OpenAI))
            .chain(Self::known_models(ProviderType::Anthropic).into_iter().map(|m| (m, ProviderType::Anthropic)))
            .chain(Self::known_models(ProviderType::DeepSeek).into_iter().map(|m| (m, ProviderType::DeepSeek)))
            .chain(Self::known_models(ProviderType::Gemini).into_iter().map(|m| (m, ProviderType::Gemini)))
            .chain(Self::known_models(ProviderType::Ollama).into_iter().map(|m| (m, ProviderType::Ollama)))
            .chain(Self::known_models(ProviderType::Mistral).into_iter().map(|m| (m, ProviderType::Mistral)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_detection() {
        assert_eq!(ProviderType::from_model("gpt-4"), ProviderType::OpenAI);
        assert_eq!(ProviderType::from_model("gpt-4-turbo"), ProviderType::OpenAI);
        assert_eq!(ProviderType::from_model("claude-3-opus"), ProviderType::Anthropic);
        assert_eq!(ProviderType::from_model("claude-3-sonnet"), ProviderType::Anthropic);
        assert_eq!(ProviderType::from_model("deepseek-chat"), ProviderType::DeepSeek);
        assert_eq!(ProviderType::from_model("gemini-pro"), ProviderType::Gemini);
        assert_eq!(ProviderType::from_model("mistral-large"), ProviderType::Mistral);
    }

    #[test]
    fn test_model_config_builder() {
        let config = ModelConfig::new("gpt-4")
            .with_base_url("https://api.openai.com/v1")
            .with_api_key_env("OPENAI_API_KEY");

        assert_eq!(config.model, "gpt-4");
        assert!(config.api_base_url.is_some());
        assert!(config.api_key_env.is_some());
    }

    #[test]
    fn test_known_models() {
        let openai_models = ProviderFactory::known_models(ProviderType::OpenAI);
        assert!(openai_models.contains(&"gpt-4"));

        let anthropic_models = ProviderFactory::known_models(ProviderType::Anthropic);
        assert!(anthropic_models.iter().any(|m| m.contains("claude")));
    }
}
