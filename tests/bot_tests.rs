//! Bot API tests

#[cfg(test)]
mod tests {
    use telegram_rs::BotClient;

    #[test]
    fn test_bot_client_creation() {
        let bot = BotClient::new("token123".to_string());
        assert_eq!(bot.token(), "token123");
    }

    // TODO: Add integration tests with Telegram test bots
    // TODO: Add tests for sendMessage, editMessage, etc.
    // TODO: Add tests for polling and webhook handling
}
