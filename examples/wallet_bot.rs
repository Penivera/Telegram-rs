//! Example: Bot with Wallet Integration
//!
//! This example demonstrates how to integrate TON wallet functionality into a Telegram bot.
//! It shows wallet connection setup and transaction handling.
//!
//! # Setup
//! Run with wallet feature: cargo run --example wallet_bot --features wallet

#[cfg(feature = "wallet")]
use telegram_rs_2::wallet::{TonConnectConfig, TonConnector};

fn main() {
    #[cfg(feature = "wallet")]
    {
        println!("Bot with Wallet Integration Example");
        let config = TonConnectConfig::new("https://example.com/manifest.json".to_string());
        let connector = TonConnector::new(config);
        println!(
            "Initialized TON Connector with manifest: {}",
            connector.config().manifest_url
        );
    }

    #[cfg(not(feature = "wallet"))]
    {
        println!("Wallet example requires 'wallet' feature");
        println!("Run with: cargo run --example wallet_bot --features wallet");
    }
}
