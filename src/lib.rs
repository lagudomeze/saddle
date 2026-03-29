mod cli;

pub use cli::Cli;

mod config;
pub use config::{ConfigLoader, Settings, init_logging};

mod harness;
pub use harness::{FeatureManager, ProgressTracker, HandoffGenerator};

mod llm;
pub use llm::{LlmClient, LlmAdapter, AgentBuilder, ProviderType, ModelConfig, ProviderFactory, OpenAiProvider};

mod agent;
pub use agent::{AgentExecutor, presets, ToolDefinition};

mod memory;
pub use memory::MemoryStore;

mod plugins;
pub use plugins::PluginRuntime;

mod tui;
pub use tui::TuiApp;

pub mod utils;

pub use utils::SaddleError;
pub use utils::SaddleResult;
