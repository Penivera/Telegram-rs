//! TON Connect v2 connector and transaction configuration models.
//!
//! Provides support for TON blockchain wallet interactions, connection URL generation,
//! and transaction parameters.

use crate::Result;
use serde::{Deserialize, Serialize};

/// Configuration parameters for TON Connect v2 integration.
///
/// Contains the manifest URL pointing to your dApp metadata (name, icon, terms of use)
/// and an optional wallet list URL.
///
/// # Examples
///
/// ```
/// use telegram_rs_2::wallet::TonConnectConfig;
///
/// let config = TonConnectConfig::new("https://example.com/tonconnect-manifest.json".to_string())
///     .with_wallet_list("https://ton-connect.github.io/wallets-list/wallets.json".to_string());
///
/// assert_eq!(config.manifest_url, "https://example.com/tonconnect-manifest.json");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TonConnectConfig {
    /// URL pointing to the dApp manifest JSON file.
    pub manifest_url: String,

    /// URL to fetch the list of supported TON wallets (defaults to the official TON Connect registry if omitted).
    pub wallet_list_url: Option<String>,
}

impl TonConnectConfig {
    /// Creates a new `TonConnectConfig` with the specified dApp manifest URL.
    ///
    /// # Arguments
    /// * `manifest_url` - Publicly accessible HTTPS URL of the dApp manifest.
    pub fn new(manifest_url: String) -> Self {
        TonConnectConfig {
            manifest_url,
            wallet_list_url: None,
        }
    }

    /// Sets a custom wallet registry list URL.
    ///
    /// # Arguments
    /// * `url` - URL returning the JSON list of supported wallets.
    pub fn with_wallet_list(mut self, url: String) -> Self {
        self.wallet_list_url = Some(url);
        self
    }
}

/// Core manager for TON Connect v2 protocol operations.
///
/// Coordinates wallet connection workflows, transaction dispatching, and manifest verification.
pub struct TonConnector {
    config: TonConnectConfig,
}

impl TonConnector {
    /// Creates a new `TonConnector` with the provided configuration.
    ///
    /// # Arguments
    /// * `config` - An instance of [`TonConnectConfig`].
    pub fn new(config: TonConnectConfig) -> Self {
        TonConnector { config }
    }

    /// Returns a reference to the active [`TonConnectConfig`].
    pub fn config(&self) -> &TonConnectConfig {
        &self.config
    }

    /// Generates a universal TON Connect deep link URL to prompt the user to connect their wallet.
    pub fn generate_connection_url(&self) -> Result<String> {
        todo!("Connection URL generation not yet implemented")
    }

    /// Generates a PNG QR code byte buffer representing the wallet connection request.
    pub fn generate_qr_code(&self) -> Result<Vec<u8>> {
        todo!("QR code generation not yet implemented")
    }

    /// Initializes a new wallet session for an incoming user connection.
    pub async fn create_session(&self) -> Result<super::session::WalletSession> {
        todo!("Session creation not yet implemented")
    }

    /// Requests the connected wallet address from an active session.
    pub async fn request_address(&self) -> Result<String> {
        todo!("Address request not yet implemented")
    }

    /// Requests the user to sign and dispatch a transfer transaction.
    ///
    /// # Arguments
    /// * `destination` - Target TON wallet address (friendly or raw hex format).
    /// * `amount` - Amount in nanotons (1 TON = 1,000,000,000 nanotons).
    pub async fn send_transaction(&self, _destination: &str, _amount: u64) -> Result<String> {
        todo!("Transaction sending not yet implemented")
    }

    /// Validates that the configured manifest URL is reachable and structurally sound.
    pub async fn validate_manifest(&self) -> Result<bool> {
        todo!("Manifest validation not yet implemented")
    }
}

/// Parameters for constructing a TON blockchain transfer transaction.
///
/// # Examples
///
/// ```
/// use telegram_rs_2::wallet::TransactionParams;
///
/// let params = TransactionParams::new("EQCD39VS5jcptHL8vMjEXrzGaRcCVYto7HUn4bpAOg8xqB2N".to_string(), 1_000_000_000)
///     .with_message("Payment for Order #1042".to_string());
///
/// assert_eq!(params.amount, 1_000_000_000);
/// assert_eq!(params.message.as_deref(), Some("Payment for Order #1042"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionParams {
    /// Target destination TON wallet address.
    pub destination: String,

    /// Amount to transfer, specified in nanotons.
    pub amount: u64,

    /// Optional plain-text payload or comment attached to the transaction.
    pub message: Option<String>,
}

impl TransactionParams {
    /// Creates a new `TransactionParams` instance.
    ///
    /// # Arguments
    /// * `destination` - Recipient TON address.
    /// * `amount` - Amount in nanotons.
    pub fn new(destination: String, amount: u64) -> Self {
        TransactionParams {
            destination,
            amount,
            message: None,
        }
    }

    /// Attaches an optional text comment or memo to the transfer.
    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}
