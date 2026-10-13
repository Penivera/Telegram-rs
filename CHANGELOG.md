# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Standard GitHub Actions CI workflows for test, fmt, clippy, doc, and security audit.
- Full community files: `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `SECURITY.md`, and issue/PR templates.
- Industry-standard dual licensing documentation for MIT and Apache-2.0.
- Re-exports of `UserData`, `SessionManager`, `TonConnectConfig`, and `TransactionParams`.
- Integration tests in `tests/` for Mini App InitData and TON Wallet sessions.

### Changed
- Replaced GNU GPLv3 license with official Apache-2.0 / MIT dual licensing notice.
- Refactored wallet session timestamps to use standard library `SystemTime`.
- Updated `Cargo.toml` with complete crates.io metadata (repository, documentation, homepage, keywords, categories, MSRV).

### Fixed
- Fixed return type incompatibility in `examples/webhook_bot.rs`.
- Fixed missing `PartialEq` implementation on `WalletSession`.
- Removed inline test blocks from source files to adhere to separate test module standards.

## [0.1.1] - 2026-01-31

### Added
- HMAC-SHA256 based cryptographic signature verification for Telegram Mini App `initData`.
- Keyboard builders: `InlineKeyboardBuilder` and `ReplyKeyboardBuilder`.
- Serialization fixes for inline query and callback button types.

## [0.1.0] - 2026-01-30

### Added
- Initial release of `telegram-rs`.
- Asynchronous Telegram Bot API client using Tokio and Reqwest.
- Long polling runtime implementation (`Polling`).
- Feature-gated Actix-Web webhook server (`webhook`).
- Procedural macros crate (`telegram-macros`) for bot handlers.
- Preliminary Mini App utilities and TON Connect v2 wallet types.
