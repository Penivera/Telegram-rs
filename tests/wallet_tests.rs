#![cfg(feature = "wallet")]

#[cfg(test)]
mod tests {
    use telegram_rs_2::wallet::{
        SessionManager, TonConnectConfig, TonConnector, TransactionParams, WalletSession,
    };

    #[test]
    fn test_ton_connect_config() {
        let config = TonConnectConfig::new("https://example.com/manifest.json".to_string())
            .with_wallet_list("https://example.com/wallets.json".to_string());
        assert_eq!(config.manifest_url, "https://example.com/manifest.json");
        assert_eq!(
            config.wallet_list_url.as_deref(),
            Some("https://example.com/wallets.json")
        );
    }

    #[test]
    fn test_ton_connector_instantiation() {
        let config = TonConnectConfig::new("https://example.com/manifest.json".to_string());
        let connector = TonConnector::new(config);
        assert_eq!(
            connector.config().manifest_url,
            "https://example.com/manifest.json"
        );
    }

    #[test]
    fn test_transaction_params() {
        let params = TransactionParams::new("addr1".to_string(), 1000)
            .with_message("test payment".to_string());
        assert_eq!(params.destination, "addr1");
        assert_eq!(params.amount, 1000);
        assert_eq!(params.message.as_deref(), Some("test payment"));
    }

    #[test]
    fn test_wallet_session_lifecycle() {
        let mut session = WalletSession::new("session123".to_string());
        assert_eq!(session.session_id, "session123");
        assert!(session.is_active);
        assert!(session.is_valid());

        session.set_wallet_address("EQAddr...".to_string());
        assert_eq!(session.wallet_address.as_deref(), Some("EQAddr..."));

        session.close();
        assert!(!session.is_active);
        assert!(!session.is_valid());
    }

    #[test]
    fn test_session_manager() {
        let mut manager = SessionManager::new();
        assert_eq!(manager.get_session("none"), None);

        let session = manager.create_session().expect("Failed to create session");
        assert!(session.is_active);
        let id = session.session_id.clone();

        assert!(manager.get_session(&id).is_some());
        manager.cleanup_expired_sessions();
        assert!(manager.get_session(&id).is_some());
    }
}
