# telegram-rs

A comprehensive Rust library for building Telegram bots and Mini Apps with optional wallet integration.

## Features

- **Telegram Bot API**: Asynchronous client for sending/receiving messages, handling updates via polling or webhooks
- **Telegram Mini Apps** (feature-gated): Utilities for creating web apps within Telegram with initData validation and WASM bridge support
- **Wallet Integration** (feature-gated): TON Connect v2 support for blockchain-enabled bots and Mini Apps

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
telegram-rs = "0.1"

# With Mini App support
telegram-rs = { version = "0.1", features = ["mini-app"] }

# With Wallet integration
telegram-rs = { version = "0.1", features = ["wallet"] }

# With all features
telegram-rs = { version = "0.1", features = ["mini-app", "wallet"] }
```

## Quick Start

### Basic Bot

```rust
use telegram_rs::{Bot, Result};
use telegram_rs::rt::polling::Polling;

#[tokio::main]
async fn main() -> Result<()> {
    let token = "YOUR_BOT_TOKEN".to_string();
    let bot = Bot::new(token);
    let mut polling = Polling::new(bot);

    while let Some(update) = polling.next_update().await? {
        println!("Received update: {:?}", update);
    }
    Ok(())
}
```

### Mini App (with `mini-app` feature)

```rust
use telegram_rs::mini_app::InitData;

fn main() {
    let raw_init_data = "query_id=123&...";
    match InitData::parse(raw_init_data) {
        Ok(init_data) => {
            if let Ok(valid) = init_data.validate("bot_token") {
                println!("InitData is valid!");
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Wallet Integration (with `wallet` feature)

```rust
use telegram_rs::wallet::{TonConnector, TonConnectConfig};

#[tokio::main]
async fn main() {
    let config = TonConnectConfig::new("https://example.com/manifest.json".to_string());
    let connector = TonConnector::new(config);
    
    // Generate connection URL for users
    if let Ok(url) = connector.generate_connection_url() {
        println!("Connect wallet at: {}", url);
    }
}
```

## Project Structure

```
telegram-rs/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── client.rs           # HTTP client implementation
│   ├── core/               # Core types and traits
│   │   ├── bot.rs          # Bot struct
│   │   ├── types.rs        # Telegram API types
│   │   └── requests.rs     # Method traits
│   ├── rt/                 # Runtime modules (Polling, Webhook)
│   │   ├── polling.rs      # Long-polling implementation
│   │   └── webhook.rs      # Webhook server (actix-web)
│   ├── mini_app/           # Mini App module (feature-gated)
│   ├── wallet/             # Wallet module (feature-gated)
│   └── errors.rs           # Custom error types
├── telegram-macros/        # Procedural macros crate
├── examples/               # Example applications
├── tests/                  # Integration tests
└── benches/                # Benchmarks
```

## Examples

Run examples with:

```bash
# Basic bot
cargo run --example simple_bot

# Weather bot (requires OPENWEATHER_API_KEY env var)
cargo run --example weather_bot

# Mini App (requires mini-app feature)
cargo run --example mini_app --features mini-app

# Wallet integration (requires wallet feature)
cargo run --example wallet_bot --features wallet
```

## Development

### Running Tests

```bash
cargo test
cargo test --features mini-app
cargo test --features wallet
```

### Running Benchmarks

```bash
cargo bench
```

### Building Documentation

```bash
cargo doc --open
```

## Roadmap

- **Phase 1**: Core Bot API client and event handling
- **Phase 2**: Mini App utilities and webhook support
- **Phase 3**: Wallet connect add-on with testing
- **Phase 4**: Enhancements based on feedback

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

- **Documentation**: [API Docs](https://docs.rs/telegram-rs)
- **Issues**: [GitHub Issues](https://github.com/yourusername/telegram-rs/issues)
- **Telegram Community**: (Link to community group)

## Acknowledgments

This library is inspired by the Telegram Bot API and aims to provide a user-friendly Rust interface for Telegram bot and Mini App development.
