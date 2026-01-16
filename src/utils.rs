//! Shared utilities for telegram-rs
//!
//! This module contains common utilities used across the library,
//! such as QR code generation, cryptographic helpers, and data conversion functions.

/// QR code generation utilities (placeholder for future implementation)
pub mod qr {
    /// Generate a QR code from data
    /// TODO: Implement QR code generation
    pub fn generate_qr(_data: &str) -> crate::Result<Vec<u8>> {
        todo!("QR code generation not yet implemented")
    }
}

/// Cryptographic utilities (placeholder for future implementation)
pub mod crypto {
    /// Hash data using SHA-256
    /// TODO: Implement hashing
    pub fn hash_sha256(_data: &[u8]) -> crate::Result<Vec<u8>> {
        todo!("SHA-256 hashing not yet implemented")
    }

    /// Verify a signature
    /// TODO: Implement signature verification
    pub fn verify_signature(_data: &[u8], _signature: &[u8], _public_key: &[u8]) -> crate::Result<bool> {
        todo!("Signature verification not yet implemented")
    }
}

/// Data validation utilities
pub mod validation {
    /// Validate a Telegram bot token format
    pub fn is_valid_bot_token(token: &str) -> bool {
        // Bot tokens are typically in format: {bot_id}:{token_string}
        let parts: Vec<&str> = token.split(':').collect();
        parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty()
    }

    /// Validate a Telegram user ID
    pub fn is_valid_user_id(user_id: i64) -> bool {
        user_id > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bot_token() {
        assert!(validation::is_valid_bot_token("123456789:ABCdefGHIjklmnoPQRstuvWXYZ"));
    }

    #[test]
    fn test_invalid_bot_token() {
        assert!(!validation::is_valid_bot_token("invalid_token"));
    }

    #[test]
    fn test_valid_user_id() {
        assert!(validation::is_valid_user_id(12345));
    }

    #[test]
    fn test_invalid_user_id() {
        assert!(!validation::is_valid_user_id(-1));
    }
}
