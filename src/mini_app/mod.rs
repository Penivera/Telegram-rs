//! Telegram Mini App module (requires "mini-app" feature)
//!
//! This module provides utilities for building Telegram Mini Apps (web apps).
//! It handles initData validation, WASM bridge interactions, and client-side integrations.

pub mod bridge;
pub mod init;

pub use init::InitData;
