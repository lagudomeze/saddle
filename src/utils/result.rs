use std::fmt;

pub type SaddleResult<T> = std::result::Result<T, SaddleError>;

#[derive(Debug)]
pub enum SaddleError {
    Io(std::io::Error),
    Parse(String),
    Plugin(String),
}

impl fmt::Display for SaddleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Parse(s) => write!(f, "Parse error: {}", s),
            Self::Plugin(s) => write!(f, "Plugin error: {}", s),
        }
    }
}
