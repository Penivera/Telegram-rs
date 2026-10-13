//! Bot API tests

#[cfg(test)]
mod tests {
    use telegram_rs_2::Bot;

    #[test]
    fn test_bot_creation() {
        let _bot = Bot::new("token123".to_string());
        // Bot struct doesn't expose token getter publicly currently,
        // so we just verify instantiation doesn't panic.
    }

    // TODO: Add integration tests with Telegram test bots
    // TODO: Add tests for sendMessage, editMessage, etc.
    // TODO: Add tests for polling and webhook handling
}
