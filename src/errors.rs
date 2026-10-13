//! Central error and result types for `telegram-rs`.

use thiserror::Error;

/// A specialized [`Result`](std::result::Result) type for `telegram-rs` operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur during `telegram-rs` operations.
#[derive(Debug, Error)]
pub enum Error {
    /// An HTTP transport or network connectivity error occurred.
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    /// A JSON serialization or deserialization error occurred.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// A standard I/O error occurred (e.g. during webhook server binding).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The Telegram Bot API returned an error response (`ok: false`).
    #[error("Telegram API error: {description} (error_code: {error_code:?})")]
    Api {
        /// HTTP or Telegram error status code, if provided.
        error_code: Option<i32>,
        /// Human-readable description of the error from Telegram.
        description: String,
    },

    /// The provided client or connector configuration is invalid.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    /// Other runtime, validation, or cryptographic error.
    #[error("Other error: {0}")]
    Other(String),
}
