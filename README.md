<div align="center">

# telegram-rs

**A fast, ergonomic, and asynchronous Rust framework for building Telegram Bots, Mini Apps, and TON Connect wallet integrations.**

[![Crates.io](https://img.shields.io/crates/v/telegram-rs-2.svg?style=flat-square&logo=rust)](https://crates.io/crates/telegram-rs-2)
[![Documentation](https://img.shields.io/docsrs/telegram-rs-2?style=flat-square&logo=docsdotrs)](https://docs.rs/telegram-rs-2)
[![Crates.io Downloads](https://img.shields.io/crates/d/telegram-rs-2.svg?style=flat-square)](https://crates.io/crates/telegram-rs-2)
[![CI](https://img.shields.io/github/actions/workflow/status/Penivera/Telegram-rs/ci.yml?branch=main&style=flat-square&logo=github)](https://github.com/Penivera/Telegram-rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg?style=flat-square)](LICENSE)
[![MSRV](https://img.shields.io/badge/MSRV-1.75.0-orange?style=flat-square&logo=rust)](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)

[Crate](https://crates.io/crates/telegram-rs-2) • [Documentation](https://docs.rs/telegram-rs-2) • [Repository](https://github.com/Penivera/Telegram-rs) • [Issues](https://github.com/Penivera/Telegram-rs/issues) • [Changelog](CHANGELOG.md)

</div>

---

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Feature Flags](#feature-flags)
- [Installation](#installation)
- [Quick Start](#quick-start)
  - [1. Echo & Button Bot (Polling)](#1-echo--button-bot-polling)
  - [2. Mini App InitData Validation](#2-mini-app-initdata-validation)
  - [3. TON Connect 2.0 Wallet Integration](#3-ton-connect-20-wallet-integration)
  - [4. Webhook Server](#4-webhook-server)
- [Repository Structure](#repository-structure)
- [Examples](#examples)
- [Development & Testing](#development--testing)
- [Community & Contributing](#community--contributing)
- [Security](#security)
- [License](#license)

---

## Overview

`telegram-rs` provides an all-in-one, modern toolkit for building high-performance Telegram bots and web-embedded Telegram Mini Apps (TMAs). Built on top of [Tokio](https://tokio.rs/) and [Reqwest](https://docs.rs/reqwest), it offers high concurrency, type-safe API requests, fluent keyboard builders, cryptographic `initData` signature validation, and optional TON Connect v2 blockchain wallet integrations.

---

## Key Features

- **Asynchronous & Type-Safe**: Fully async non-blocking runtime powered by Tokio and Reqwest.
- **Telegram Bot API**: Send messages, handle updates via long-polling or webhooks, answer callback queries, and construct interactive keyboards.
- **Fluent UI Builders**: Easily create `InlineKeyboardMarkup` and `ReplyKeyboardMarkup` using declarative builder patterns (`InlineKeyboardBuilder`, `ReplyKeyboardBuilder`).
- **Telegram Mini Apps (`mini-app`)**: Parse launch data, perform HMAC-SHA256 signature verification against your bot token, check authentication freshness timestamps, and bridge with client environments.
- **TON Connect 2.0 (`wallet`)**: Native structs and session lifecycle manager for connecting TON blockchain wallets.
- **Webhook Engine (`webhook`)**: Built-in production-grade Actix-Web server integration for high-throughput webhook delivery.
- **Procedural Macros (`telegram-macros`)**: Convenient attribute macros (`#[bot]`, `#[command]`) for cleaner bot definitions.

---

## Feature Flags

`telegram-rs` is designed with modularity in mind. All optional capabilities are gated behind Cargo features so you only compile what you need:

| Feature | Description | Default | Dependencies |
| :--- | :--- | :---: | :--- |
| `bot` | Core Telegram Bot API client and polling runtime | Yes | `tokio`, `reqwest`, `serde` |
| `mini-app` | Telegram Mini App `initData` parsing & cryptographic validation | No | `hmac`, `sha2`, `hex`, `urlencoding` |
| `wallet` | TON Connect v2 wallet connectors and session management | No | `uuid` |
| `webhook` | Webhook HTTP server integration | No | `actix-web` |

---

## Installation

Add `telegram-rs-2` to your `Cargo.toml`:

```toml
[dependencies]
# Basic Bot API client
telegram-rs-2 = "0.1.1"

# With Mini App initData validation support
telegram-rs-2 = { version = "0.1.1", features = ["mini-app"] }

# With TON Connect v2 Wallet integration
telegram-rs-2 = { version = "0.1.1", features = ["wallet"] }

# With all features enabled
telegram-rs-2 = { version = "0.1.1", features = ["mini-app", "wallet", "webhook"] }
```

---

## Quick Start

### 1. Echo & Button Bot (Polling)

```rust
use telegram_rs_2::core::requests::{AnswerCallbackQuery, SendMessage};
use telegram_rs_2::core::types::{InlineKeyboardBuilder, InlineKeyboardButton};
use telegram_rs_2::rt::polling::Polling;
use telegram_rs_2::{Bot, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN")
        .expect("TELEGRAM_BOT_TOKEN environment variable not set");

    let bot = Bot::new(token);
    let mut polling = Polling::new(bot.clone());

    println!("Bot started! Polling for updates...");

    while let Some(update) = polling.next_update().await? {
        // Handle incoming text message
        if let Some(msg) = update.message {
            if let Some(text) = msg.text {
                let keyboard = InlineKeyboardBuilder::new()
                    .row(vec![
                        InlineKeyboardButton::callback("Help", "help_btn"),
                        InlineKeyboardButton::url("Website", "https://telegram.org"),
                    ])
                    .build();

                let send_msg = SendMessage {
                    chat_id: msg.chat.id,
                    text: format!("Echo: {}", text),
                    parse_mode: None,
                    reply_markup: Some(keyboard),
                };

                bot.execute(send_msg).await?;
            }
        }

        // Handle inline button clicks
        if let Some(callback) = update.callback_query {
            let answer = AnswerCallbackQuery {
                callback_query_id: callback.id,
                text: Some("Button acknowledged!".to_string()),
                show_alert: Some(false),
                url: None,
                cache_time: None,
            };

            bot.execute(answer).await?;
        }
    }

    Ok(())
}
```

### 2. Mini App InitData Validation

```rust
#[cfg(feature = "mini-app")]
use telegram_rs_2::mini_app::InitData;

fn validate_webapp_launch(raw_init_data: &str, bot_token: &str) -> bool {
    match InitData::parse(raw_init_data) {
        Ok(data) => {
            // 1. Verify HMAC-SHA256 signature against the bot token
            let is_signature_valid = data.validate(bot_token).unwrap_or(false);

            // 2. Ensure launch data is not older than 5 minutes (300 seconds)
            let is_fresh = data.is_valid_timestamp(300);

            if is_signature_valid && is_fresh {
                if let Some(user) = &data.user {
                    println!("Authenticated user: {} (ID: {})", user.first_name, user.id);
                }
                true
            } else {
                false
            }
        }
        Err(err) => {
            eprintln!("Failed to parse initData: {}", err);
            false
        }
    }
}
```

### 3. TON Connect 2.0 Wallet Integration

```rust
#[cfg(feature = "wallet")]
use telegram_rs_2::wallet::{SessionManager, TonConnectConfig, TonConnector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure TON Connect with your app's manifest URL
    let config = TonConnectConfig::new("https://my-app.com/tonconnect-manifest.json".to_string())
        .with_wallet_list("https://ton-connect.github.io/wallets-list/wallets.json".to_string());

    let connector = TonConnector::new(config);
    println!("Manifest URL: {}", connector.config().manifest_url);

    // Manage client wallet connection sessions
    let mut manager = SessionManager::new();
    let session = manager.create_session()?;
    println!("Created new wallet session: {}", session.session_id);

    Ok(())
}
```

### 4. Webhook Server

```rust
#[cfg(feature = "webhook")]
use telegram_rs_2::core::types::Update;
#[cfg(feature = "webhook")]
use telegram_rs_2::rt::webhook::Webhook;
#[cfg(feature = "webhook")]
use telegram_rs_2::{Bot, Result};

#[cfg(feature = "webhook")]
#[tokio::main]
async fn main() -> Result<()> {
    let token = std::env::var("TELEGRAM_BOT_TOKEN").expect("Token missing");
    let bot = Bot::new(token);

    println!("Starting webhook server on port 8080...");
    Webhook::new(bot)
        .port(8080)
        .run(|update: Update, _bot: Bot| async move {
            println!("Received update via webhook: {:?}", update);
        })
        .await?;

    Ok(())
}
```

---

## Repository Structure

```
telegram-rs/
├── .github/                     # GitHub standards and CI workflows
│   ├── ISSUE_TEMPLATE/          # Structured issue forms (Bug report, Feature request)
│   ├── workflows/               # GitHub Actions (CI & security audit)
│   └── pull_request_template.md # PR guidelines and checklist
├── benches/                     # Benchmark suite
├── docs/                        # Interactive documentation web app (React + Vite)
├── examples/                    # Executable reference examples
│   ├── simple_bot.rs            # Long-polling bot with inline buttons
│   ├── mini_app.rs              # Mini App initData parsing
│   ├── wallet_bot.rs            # TON Connect wallet initialization
│   └── webhook_bot.rs           # Actix-Web webhook server
├── src/                         # Core library implementation
│   ├── core/                    # Core types, API methods, and builders
│   ├── mini_app/                # Telegram Mini App modules
│   ├── rt/                      # Polling & webhook runtime engines
│   ├── wallet/                  # TON Connect integration
│   ├── client.rs                # Async HTTP transport
│   ├── errors.rs                # Custom Error and Result types
│   └── lib.rs                   # Main crate re-exports
├── telegram-macros/             # Procedural attribute macros
├── tests/                       # Integration test suites
│   ├── bot_tests.rs
│   ├── init_data_tests.rs
│   ├── mini_app_tests.rs
│   └── wallet_tests.rs
├── CHANGELOG.md                 # Semantic version changelog
├── CODE_OF_CONDUCT.md           # Contributor Covenant v2.1
├── CONTRIBUTING.md               # Contributing guide and local setup
├── LICENSE-APACHE               # Apache 2.0 License
├── LICENSE-MIT                  # MIT License
└── SECURITY.md                  # Vulnerability disclosure policy
```

---

## Examples

Runnable example applications are included in the [`examples/`](examples) directory:

```bash
# 1. Interactive Polling Bot with buttons
TELEGRAM_BOT_TOKEN="your_token" cargo run --example simple_bot

# 2. Weather Bot (requires OPENWEATHER_API_KEY env var)
OPENWEATHER_API_KEY="your_api_key" TELEGRAM_BOT_TOKEN="your_token" cargo run --example weather_bot

# 3. Mini App initData validation
cargo run --example mini_app --features mini-app

# 4. TON Connect Wallet integration
cargo run --example wallet_bot --features wallet

# 5. Webhook Server
TELEGRAM_BOT_TOKEN="your_token" cargo run --example webhook_bot --features webhook
```

---

## Development & Testing

We welcome local contributions! Ensure your environment has the Rust stable toolchain installed.

```bash
# Check compilation across all targets and features
cargo check --all-targets --all-features

# Run the test suite
cargo test --all-targets --all-features

# Verify formatting
cargo fmt --all -- --check

# Run Clippy with warnings denied
cargo clippy --all-targets --all-features -- -D warnings

# Build documentation locally
cargo doc --no-deps --all-features --open
```

---

## Community & Contributing

Contributions are what make the open-source community an incredible place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

- Please read our [**Contributing Guide**](CONTRIBUTING.md) for branch naming, testing standards, and pull request procedures.
- We adhere to the [**Contributor Covenant Code of Conduct**](CODE_OF_CONDUCT.md).
- Have a question or proposal? Start a discussion in [GitHub Discussions](https://github.com/Penivera/Telegram-rs/discussions) or open an issue.

---

## Security

If you discover a security vulnerability within `telegram-rs`, please review our [**Security Policy**](SECURITY.md) and report it privately via GitHub Security Advisories rather than public issue trackers.

---

## License

`telegram-rs` is dual-licensed under either:

* **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `telegram-rs` by you shall be dual-licensed as above, without any additional terms or conditions.
