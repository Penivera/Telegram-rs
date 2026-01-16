//! Custom error types for telegram-rs

use std::fmt;

/// A specialized Result type for telegram-rs operations
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur during telegram-rs operations
#[derive(Debug)]
pub enum Error {
    /// HTTP request error
    HttpError(String),
    /// JSON serialization/deserialization error
    JsonError(String),
    /// Invalid configuration or parameters
    InvalidConfig(String),
    /// API error from Telegram
    ApiError { code: i32, message: String },
    /// Authentication error
    AuthError(String),
    /// Wallet operation error
    WalletError(String),
    /// Other errors
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::HttpError(msg) => write!(f, "HTTP Error: {}", msg),
            Error::JsonError(msg) => write!(f, "JSON Error: {}", msg),
            Error::InvalidConfig(msg) => write!(f, "Invalid Config: {}", msg),
            Error::ApiError { code, message } => write!(f, "API Error ({}): {}", code, message),
            Error::AuthError(msg) => write!(f, "Auth Error: {}", msg),
            Error::WalletError(msg) => write!(f, "Wallet Error: {}", msg),
            Error::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
