//! TON Connect v2 blockchain wallet integration for Telegram bots and Mini Apps.
//!
//! Available when the `wallet` Cargo feature is enabled.
//!
//! This module provides primitives to connect Telegram applications to the TON blockchain,
//! including manifest configuration ([`TonConnectConfig`]), connector initialization ([`TonConnector`]),
//! and session lifecycle management ([`WalletSession`], [`SessionManager`]).

pub mod session;
pub mod ton;

pub use session::{SessionManager, WalletSession};
pub use ton::{TonConnectConfig, TonConnector, TransactionParams};
