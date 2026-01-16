//! # telegram-rs
//!
//! A comprehensive Rust library for building Telegram bots and Mini Apps with optional wallet integration.
//!
//! ## Features
//!
//! - **Bot API**: Asynchronous client for sending/receiving messages, handling updates via polling or webhooks
//! - **Mini Apps** (feature-gated): Utilities for Telegram Web Apps with initData validation
//! - **Wallet Connect** (feature-gated): TON Connect v2 integration for blockchain functionality
//!
//! ## Quick Start
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! telegram-rs = "0.1"
//! ```

pub mod bot;
pub mod errors;
pub mod utils;

#[cfg(feature = "mini-app")]
pub mod mini_app;

#[cfg(feature = "wallet")]
pub mod wallet;

// Re-export commonly used types
pub use bot::client::BotClient;
pub use errors::{Error, Result};
