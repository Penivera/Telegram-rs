//! Wallet integration module (requires "wallet" feature)
//!
//! This module provides TON Connect v2 integration for wallet operations.
//! Allows bots and Mini Apps to interact with TON wallets for transactions and signing.

pub mod session;
pub mod ton;

pub use session::WalletSession;
pub use ton::TonConnector;
