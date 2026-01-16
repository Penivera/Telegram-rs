//! Update handling and event dispatching
//!
//! This module handles incoming updates from Telegram and dispatches them to appropriate handlers.
//! Supports both polling and webhook-based update retrieval.

use serde::{Deserialize, Serialize};

/// Represents an update from Telegram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    /// The update's unique identifier
    pub update_id: i32,
    // TODO: Add other update fields (message, callback_query, etc.)
}

/// Result of processing an update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    pub success: bool,
    pub message: Option<String>,
}

/// Polling configuration for update retrieval
pub struct PollingConfig {
    /// Timeout for long polling (in seconds)
    pub timeout: u32,
    /// Allowed update types (empty = all)
    pub allowed_updates: Vec<String>,
}

impl Default for PollingConfig {
    fn default() -> Self {
        PollingConfig {
            timeout: 30,
            allowed_updates: vec![],
        }
    }
}

/// Webhook configuration for update retrieval
pub struct WebhookConfig {
    /// Webhook URL
    pub url: String,
    /// Optional certificate path
    pub certificate: Option<String>,
    /// Allowed update types (empty = all)
    pub allowed_updates: Vec<String>,
}

// TODO: Implement polling loop
// TODO: Implement webhook server
// TODO: Implement update dispatcher

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_creation() {
        let update = Update { update_id: 1 };
        assert_eq!(update.update_id, 1);
    }

    #[test]
    fn test_polling_config_default() {
        let config = PollingConfig::default();
        assert_eq!(config.timeout, 30);
    }
}
