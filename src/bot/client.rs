//! Async client for the Telegram Bot API
//!
//! This module provides the main BotClient struct for interacting with Telegram's Bot API.
//! It supports sending messages, handling updates via polling/webhooks, and other bot operations.

use crate::Result;

/// The main async client for interacting with Telegram Bot API
pub struct BotClient {
    token: String,
    // TODO: Add reqwest::Client for HTTP operations
}

impl BotClient {
    /// Create a new BotClient with the given bot token
    ///
    /// # Arguments
    /// * `token` - The bot token from BotFather
    ///
    /// # Example
    /// ```
    /// # use telegram_rs::BotClient;
    /// let bot = BotClient::new("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string());
    /// ```
    pub fn new(token: String) -> Self {
        BotClient { token }
    }

    /// Get the bot token
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Send a message to a chat
    /// TODO: Implement sendMessage API method
    pub async fn send_message(&self, chat_id: i64, text: &str) -> Result<()> {
        todo!("send_message not yet implemented")
    }

    /// Edit a message
    /// TODO: Implement editMessage API method
    pub async fn edit_message(&self, chat_id: i64, message_id: i32, text: &str) -> Result<()> {
        todo!("edit_message not yet implemented")
    }

    /// Send a photo
    /// TODO: Implement sendPhoto API method
    pub async fn send_photo(&self, chat_id: i64, photo_url: &str) -> Result<()> {
        todo!("send_photo not yet implemented")
    }

    /// Get updates from Telegram (polling method)
    /// TODO: Implement getUpdates API method
    pub async fn get_updates(&self, offset: i32) -> Result<Vec<crate::bot::Update>> {
        todo!("get_updates not yet implemented")
    }

    /// Start polling for updates
    /// TODO: Implement polling loop
    pub async fn start_polling<H: crate::bot::UpdateHandler>(&self, handler: H) -> Result<()> {
        todo!("start_polling not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_client_creation() {
        let bot = BotClient::new("token123".to_string());
        assert_eq!(bot.token(), "token123");
    }
}
