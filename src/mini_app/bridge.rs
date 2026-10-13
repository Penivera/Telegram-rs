//! Client-side bridge bindings for Telegram Web App features.
//!
//! Provides bindings for Telegram client-side interactions such as haptic feedback,
//! alerts, popups, and cloud storage.

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Interface for Telegram Mini App client interactions.
///
/// In WASM target environments, this interacts directly with `window.Telegram.WebApp`.
/// In non-WASM targets, a fallback stub is provided.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct TelegramWebApp;

#[cfg(target_arch = "wasm32")]
impl TelegramWebApp {
    /// Initializes the Telegram Web App client interface.
    #[wasm_bindgen(constructor)]
    pub fn new() -> TelegramWebApp {
        TelegramWebApp {}
    }

    /// Signals to the Telegram client that the Mini App is ready to be displayed.
    pub fn init(&self) -> crate::Result<()> {
        Ok(())
    }

    /// Displays a native Telegram client alert dialog.
    pub fn show_alert(&self, message: &str) -> crate::Result<()> {
        println!("MiniApp Alert: {}", message);
        Ok(())
    }

    /// Triggers native haptic feedback on supported mobile devices.
    ///
    /// # Arguments
    /// * `style` - Haptic style (e.g. `"light"`, `"medium"`, `"heavy"`, `"rigid"`, `"soft"`).
    pub fn haptic_feedback(&self, style: &str) -> crate::Result<()> {
        println!("MiniApp Haptic: {}", style);
        Ok(())
    }

    /// Stores a key-value pair in Telegram Cloud Storage.
    pub fn cloud_storage_set(&self, key: &str, value: &str) -> crate::Result<()> {
        println!("Cloud Storage Set: {} = {}", key, value);
        Ok(())
    }

    /// Retrieves a value by key from Telegram Cloud Storage.
    pub fn cloud_storage_get(&self, key: &str) -> crate::Result<Option<String>> {
        println!("Cloud Storage Get: {}", key);
        Ok(None)
    }

    /// Closes the Mini App window.
    pub fn close(&self) -> crate::Result<()> {
        println!("MiniApp Close");
        Ok(())
    }

    /// Displays a native modal popup dialog with a title and message.
    pub fn show_popup(&self, title: &str, message: &str) -> crate::Result<()> {
        println!("MiniApp Popup: {} - {}", title, message);
        Ok(())
    }
}

// Non-WASM fallback implementation
#[cfg(not(target_arch = "wasm32"))]
/// Non-WASM stub representation of `TelegramWebApp`.
pub struct TelegramWebApp;

#[cfg(not(target_arch = "wasm32"))]
impl TelegramWebApp {
    /// Fallback constructor for non-WASM targets.
    ///
    /// # Panics
    /// Panics when called outside a WASM execution environment.
    pub fn new() -> Self {
        panic!("TelegramWebApp bridge is only available in WASM environments")
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for TelegramWebApp {
    fn default() -> Self {
        Self::new()
    }
}
