//! Core Telegram Bot API primitives, requests, and data models.
//!
//! This module contains the primary [`Bot`] client struct, request payloads
//! implementing the [`requests::Method`] trait, and Telegram API types.

pub mod bot;
pub mod requests;
pub mod types;

pub use bot::Bot;
