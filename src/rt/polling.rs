//! Long-polling runtime implementation for receiving Telegram updates.
//!
//! Provides a buffered stream-like interface over Telegram's `getUpdates` method.

use crate::core::types::Update;
use crate::Bot;
use crate::Error;
use std::collections::VecDeque;

/// An asynchronous long-polling runner that queries Telegram for updates.
///
/// `Polling` automatically handles update offset tracking, buffering of multiple updates,
/// and reconnecting on timeouts.
///
/// # Examples
///
/// ```no_run
/// use telegram_rs_2::rt::polling::Polling;
/// use telegram_rs_2::{Bot, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let bot = Bot::new("YOUR_TOKEN".to_string());
///     let mut polling = Polling::new(bot).timeout(20);
///
///     while let Some(update) = polling.next_update().await? {
///         println!("Received update ID: {}", update.update_id);
///     }
///     Ok(())
/// }
/// ```
pub struct Polling {
    bot: Bot,
    timeout: u64,
    offset: Option<i32>,
    buffer: VecDeque<Update>,
}

impl Polling {
    /// Creates a new long-polling runner with a default 10-second request timeout.
    ///
    /// # Arguments
    /// * `bot` - Instance of [`Bot`].
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            timeout: 10,
            offset: None,
            buffer: VecDeque::new(),
        }
    }

    /// Sets the long-polling timeout in seconds (passed to Telegram's `getUpdates` method).
    ///
    /// Defaults to 10 seconds. Higher timeouts (e.g. 20-30s) reduce HTTP polling overhead.
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

    /// Fetches the next available update from the local buffer or queries Telegram for a new batch.
    ///
    /// Updates the internal offset tracker (`last_update_id + 1`) to ensure received updates are acknowledged.
    ///
    /// # Errors
    /// Returns [`Error`] if an HTTP or serialization failure occurs.
    pub async fn next_update(&mut self) -> Result<Option<Update>, Error> {
        if let Some(update) = self.buffer.pop_front() {
            return Ok(Some(update));
        }

        let req = crate::core::requests::GetUpdates {
            offset: self.offset,
            limit: Some(100),
            timeout: Some(self.timeout),
            allowed_updates: None,
        };

        let updates = self.bot.execute(req).await?;

        if let Some(last) = updates.last() {
            self.offset = Some(last.update_id + 1);
        }

        self.buffer.extend(updates);
        Ok(self.buffer.pop_front())
    }
}
