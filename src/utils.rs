use derive_more::{Display, Error};

pub type SaddleResult<T> = exn::Result<T, SaddleError>;

#[derive(Debug, Display, Error)]
pub enum SaddleError {
    #[display("IO error: {_0}")]
    Io(std::io::Error),
    #[display("Parse error: {_0}")]
    Parse(#[error(not(source))] String),
    #[display("Plugin error: {_0}")]
    Plugin(#[error(not(source))] String),
    #[display("Config error: {_0}")]
    Config(#[error(not(source))] String),
    #[display("Logging error: {_0}")]
    Logging(#[error(not(source))] String),
    #[display("Feature error: {_0}")]
    Feature(#[error(not(source))] String),
    #[display("Progress error: {_0}")]
    Progress(#[error(not(source))] String),
    #[display("Handoff error: {_0}")]
    Handoff(#[error(not(source))] String),
    #[display("TUI error: {_0}")]
    Tui(#[error(not(source))] String),
    #[display("LLM error: {_0}")]
    Llm(#[error(not(source))] String),
    #[display("LLM adapter error: {_0}")]
    LlmAdapter(#[error(not(source))] String),
    #[display("Memory error: {_0}")]
    Memory(#[error(not(source))] String),
    #[display("Init error: {_0}")]
    Init(#[error(not(source))] String),
    #[display("Other error: {_0}")]
    Other(#[error(not(source))] String),
}

impl SaddleError {
    pub fn feature(msg: impl Into<String>) -> Self {
        SaddleError::Feature(msg.into())
    }

    pub fn progress(msg: impl Into<String>) -> Self {
        SaddleError::Progress(msg.into())
    }

    pub fn handoff(msg: impl Into<String>) -> Self {
        SaddleError::Handoff(msg.into())
    }

    pub fn config(msg: impl Into<String>) -> Self {
        SaddleError::Config(msg.into())
    }

    pub fn logging(msg: impl Into<String>) -> Self {
        SaddleError::Logging(msg.into())
    }

    pub fn tui(msg: impl Into<String>) -> Self {
        SaddleError::Tui(msg.into())
    }

    pub fn memory(msg: impl Into<String>) -> Self {
        SaddleError::Memory(msg.into())
    }

    pub fn parse(msg: impl Into<String>) -> Self {
        SaddleError::Parse(msg.into())
    }

    pub fn init(msg: impl Into<String>) -> Self {
        SaddleError::Init(msg.into())
    }

    pub fn llm(msg: impl Into<String>) -> Self {
        SaddleError::Llm(msg.into())
    }

    pub fn io(e: std::io::Error) -> Self {
        SaddleError::Io(e)
    }
}
