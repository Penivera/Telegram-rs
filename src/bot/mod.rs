//! Telegram Bot API module
//!
//! This module provides an asynchronous client for interacting with the Telegram Bot API.
//! It handles sending/receiving messages, managing updates via polling or webhooks, and dispatching events.

pub mod client;
pub mod handlers;
pub mod updates;

pub use client::BotClient;
pub use handlers::UpdateHandler;
pub use updates::Update;
