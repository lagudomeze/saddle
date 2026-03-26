use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub app: AppConfig,
    pub llm: LlmConfig,
    pub memory: MemoryConfig,
    pub plugins: PluginsConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub data_dir: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmConfig {
    pub default_model: String,
    pub api_base_url: String,
    pub api_key_env: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MemoryConfig {
    pub db_path: String,
    pub vec_dim: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PluginsConfig {
    pub plugin_dir: String,
    pub max_memory_mb: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
    pub format: String,
}

fn saddle_base_dir() -> String {
    if cfg!(windows) {
        ".saddle".to_string()
    } else {
        "~/.saddle".to_string()
    }
}

impl Default for Settings {
    fn default() -> Self {
        let base = saddle_base_dir();
        Self {
            app: AppConfig {
                name: "saddle".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                data_dir: base.clone(),
            },
            llm: LlmConfig {
                default_model: "gpt-4".to_string(),
                api_base_url: "https://api.openai.com/v1".to_string(),
                api_key_env: "OPENAI_API_KEY".to_string(),
                timeout_secs: 120,
            },
            memory: MemoryConfig {
                db_path: format!("{}/memory.db", base),
                vec_dim: 1536,
            },
            plugins: PluginsConfig {
                plugin_dir: format!("{}/plugins", base),
                max_memory_mb: 512,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: Some(format!("{}/logs/saddle.log", base)),
                format: "json".to_string(),
            },
        }
    }
}
