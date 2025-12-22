use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("LLM error: {0}")]
    Llm(String),
    #[error("Voice error: {0}")]
    Voice(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}
