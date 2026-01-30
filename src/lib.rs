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

pub mod errors;
pub mod client;
pub mod core;
pub mod rt;

pub use core::Bot;
pub use core::types::Update;

pub use self::errors::{Error, Result};
pub use telegram_macros::*;

#[cfg(feature = "mini-app")]
pub mod mini_app;

#[cfg(feature = "wallet")]
pub mod wallet;
