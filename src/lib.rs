//! # telegram-rs
//!
//! A fast, ergonomic, and asynchronous Rust framework for building Telegram Bots,
//! Telegram Mini Apps, and TON Connect wallet integrations.
//!
//! ## Overview
//!
//! `telegram-rs` provides a modular, type-safe toolkit for interacting with the
//! [Telegram Bot API](https://core.telegram.org/bots/api), validating [Telegram Mini Apps](https://core.telegram.org/bots/webapps)
//! launch parameters, and managing [TON Connect v2](https://github.com/ton-connect) wallet sessions.
//!
//! Built on top of [Tokio](https://tokio.rs/) and [Reqwest](https://docs.rs/reqwest), `telegram-rs`
//! is designed for non-blocking asynchronous execution, low memory overhead, and clear error handling.
//!
//! ## Feature Flags
//!
//! `telegram-rs` uses Cargo feature flags to allow developers to include only the dependencies
//! they need for their specific deployment:
//!
//! | Feature | Description | Default | Dependencies |
//! | :--- | :--- | :---: | :--- |
//! | **`bot`** | Core Telegram Bot API client and long-polling runtime engine | **Yes** | `tokio`, `reqwest`, `serde` |
//! | **`mini-app`** | Telegram Mini App `initData` parsing, HMAC validation, and bridge utilities | No | `hmac`, `sha2`, `hex`, `urlencoding` |
//! | **`wallet`** | TON Connect v2 integration, manifest configuration, and session management | No | `uuid` |
//! | **`webhook`** | Production Actix-Web server integration for high-throughput webhook updates | No | `actix-web` |
//!
//! To enable specific features, configure your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! # Basic bot polling
//! telegram-rs-2 = "0.1.1"
//!
//! # Mini App validation and Webhook support
//! telegram-rs-2 = { version = "0.1.1", features = ["mini-app", "webhook"] }
//!
//! # All features enabled
//! telegram-rs-2 = { version = "0.1.1", features = ["mini-app", "wallet", "webhook"] }
//! ```
//!
//! ## Quick Start Examples
//!
//! ### 1. Polling Bot with Inline Keyboard Buttons
//!
//! ```no_run
//! use telegram_rs_2::core::requests::{AnswerCallbackQuery, SendMessage};
//! use telegram_rs_2::core::types::{InlineKeyboardBuilder, InlineKeyboardButton};
//! use telegram_rs_2::rt::polling::Polling;
//! use telegram_rs_2::{Bot, Result};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let token = std::env::var("TELEGRAM_BOT_TOKEN")
//!         .expect("TELEGRAM_BOT_TOKEN must be set");
//!     let bot = Bot::new(token);
//!     let mut polling = Polling::new(bot.clone());
//!
//!     println!("Bot started! Polling for updates...");
//!
//!     while let Some(update) = polling.next_update().await? {
//!         // Handle incoming text message
//!         if let Some(msg) = update.message {
//!             if let Some(text) = msg.text {
//!                 let keyboard = InlineKeyboardBuilder::new()
//!                     .row(vec![
//!                         InlineKeyboardButton::callback("Help", "help_btn"),
//!                         InlineKeyboardButton::url("Website", "https://telegram.org"),
//!                     ])
//!                     .build();
//!
//!                 let send_msg = SendMessage {
//!                     chat_id: msg.chat.id,
//!                     text: format!("Echo: {}", text),
//!                     parse_mode: None,
//!                     reply_markup: Some(keyboard),
//!                 };
//!
//!                 bot.execute(send_msg).await?;
//!             }
//!         }
//!
//!         // Handle inline button clicks
//!         if let Some(callback) = update.callback_query {
//!             let answer = AnswerCallbackQuery {
//!                 callback_query_id: callback.id,
//!                 text: Some("Button clicked!".to_string()),
//!                 show_alert: Some(false),
//!                 url: None,
//!                 cache_time: None,
//!             };
//!
//!             bot.execute(answer).await?;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ### 2. Mini App InitData Cryptographic Validation (`mini-app`)
//!
//! Validate user launch data securely on your server using HMAC-SHA256:
//!
//! ```no_run
//! #[cfg(feature = "mini-app")]
//! {
//!     use telegram_rs_2::mini_app::InitData;
//!
//!     let raw_init_data = "query_id=123&user=%7B%22id%22%3A123%2C%22first_name%22%3A%22Alice%22%7D&auth_date=1700000000&hash=abc123hash";
//!     let bot_token = "123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11";
//!
//!     let init_data = InitData::parse(raw_init_data).expect("Malformed initData query string");
//!
//!     // 1. Verify HMAC-SHA256 signature
//!     let is_valid = init_data.validate(bot_token).unwrap_or(false);
//!
//!     // 2. Prevent replay attacks (check data was generated within the last 5 minutes)
//!     let is_fresh = init_data.is_valid_timestamp(300);
//!
//!     if is_valid && is_fresh {
//!         if let Some(user) = init_data.user {
//!             println!("Authenticated user: {} (ID: {})", user.first_name, user.id);
//!         }
//!     }
//! }
//! ```
//!
//! ### 3. TON Connect 2.0 Integration (`wallet`)
//!
//! ```no_run
//! #[cfg(feature = "wallet")]
//! {
//!     use telegram_rs_2::wallet::{SessionManager, TonConnectConfig, TonConnector};
//!
//!     // Configure TON Connect manifest and wallet registry
//!     let config = TonConnectConfig::new("https://my-app.com/tonconnect-manifest.json".to_string())
//!         .with_wallet_list("https://ton-connect.github.io/wallets-list/wallets.json".to_string());
//!     let connector = TonConnector::new(config);
//!
//!     // Manage user connection sessions
//!     let mut manager = SessionManager::new();
//!     let session = manager.create_session().expect("Failed to create session");
//!     println!("Created session: {}", session.session_id);
//! }
//! ```
//!
//! ### 4. Webhook Server (`webhook`)
//!
//! ```no_run
//! #[cfg(feature = "webhook")]
//! {
//!     use telegram_rs_2::core::types::Update;
//!     use telegram_rs_2::rt::webhook::Webhook;
//!     use telegram_rs_2::{Bot, Result};
//!
//!     #[tokio::main]
//!     async fn main() -> Result<()> {
//!         let bot = Bot::new("BOT_TOKEN".to_string());
//!         Webhook::new(bot)
//!             .port(8080)
//!             .path("/custom_webhook".to_string())
//!             .run(|update: Update, _bot: Bot| async move {
//!                 println!("Received update via webhook: {:?}", update);
//!             })
//!             .await?;
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Modules & Architecture
//!
//! - [`core`]: Core Telegram Bot API abstraction including [`Bot`], requests ([`core::requests`]), and types ([`core::types`]).
//! - [`client`]: Underlying async HTTP client powered by Reqwest.
//! - [`rt`]: Runtime engines: long polling ([`rt::polling`]) and Actix-Web webhooks ([`rt::webhook`]).
//! - [`mini_app`]: Telegram Mini App authentication and bridge utilities (`mini-app` feature).
//! - [`wallet`]: TON Connect v2 blockchain wallet integrations (`wallet` feature).
//! - [`errors`]: Central error enum ([`Error`]) and specialized [`Result`] alias.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod client;
pub mod core;
pub mod errors;
pub mod rt;

pub use core::types::Update;
pub use core::Bot;

pub use self::errors::{Error, Result};
pub use telegram_macros::*;

#[cfg(feature = "mini-app")]
pub mod mini_app;

#[cfg(feature = "wallet")]
pub mod wallet;
