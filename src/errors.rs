use thiserror::Error;

/// A specialized Result type for telegram-rs operations
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur during telegram-rs operations
#[derive(Debug, Error)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Telegram API error: {description} (error_code: {error_code:?})")]
    Api {
        error_code: Option<i32>,
        description: String,
    },

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Other error: {0}")]
    Other(String),
}

