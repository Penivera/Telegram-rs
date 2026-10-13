//! Telegram Bot API client representation.

use crate::client::HttpClient;
use crate::Error;
use std::sync::Arc;

/// The primary client for interacting with the Telegram Bot API.
///
/// `Bot` encapsulates the bot authentication token and an asynchronous HTTP transport.
/// It is cheaply cloneable because the underlying HTTP client is wrapped in an [`Arc`].
///
/// # Examples
///
/// Creating a bot with default Telegram servers:
///
/// ```no_run
/// use telegram_rs_2::Bot;
///
/// let bot = Bot::new("123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11".to_string());
/// ```
///
/// Creating a bot pointing to a local self-hosted Telegram Bot API server:
///
/// ```no_run
/// use telegram_rs_2::Bot;
///
/// let bot = Bot::with_api_url(
///     "TOKEN".to_string(),
///     "http://localhost:8081/botTOKEN/".to_string(),
/// );
/// ```
#[derive(Clone, Debug)]
pub struct Bot {
    pub(crate) client: Arc<HttpClient>,
}

impl Bot {
    /// Creates a new `Bot` instance using the default Telegram Bot API endpoint (`https://api.telegram.org/bot<token>/`).
    ///
    /// # Arguments
    ///
    /// * `token` - The Telegram Bot API authentication token obtained from [@BotFather](https://t.me/BotFather).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use telegram_rs_2::Bot;
    ///
    /// let bot = Bot::new("YOUR_TELEGRAM_BOT_TOKEN".to_string());
    /// ```
    pub fn new(token: String) -> Self {
        Self {
            client: Arc::new(HttpClient::new(token)),
        }
    }

    /// Creates a new `Bot` instance targeting a custom API base URL.
    ///
    /// Useful for self-hosted [Telegram Bot API servers](https://core.telegram.org/bots/api#using-a-local-bot-api-server),
    /// proxy services, or mock testing servers.
    ///
    /// # Arguments
    ///
    /// * `token` - The Telegram Bot API authentication token.
    /// * `api_url` - The base endpoint URL (must include trailing slash or bot path as expected by the server).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use telegram_rs_2::Bot;
    ///
    /// let bot = Bot::with_api_url(
    ///     "YOUR_BOT_TOKEN".to_string(),
    ///     "https://my-proxy.internal/botYOUR_BOT_TOKEN/".to_string(),
    /// );
    /// ```
    pub fn with_api_url(token: String, api_url: String) -> Self {
        Self {
            client: Arc::new(HttpClient::with_api_url(token, api_url)),
        }
    }

    /// Sends a strongly typed API request method to the Telegram Bot API server.
    ///
    /// The method must implement the [`Method`](crate::core::requests::Method) trait and [`serde::Serialize`].
    /// Responses are automatically deserialized into the associated `M::Response` type.
    ///
    /// # Arguments
    ///
    /// * `method` - An instance of a method struct, such as [`SendMessage`](crate::core::requests::SendMessage) or [`GetUpdates`](crate::core::requests::GetUpdates).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Network`] if an HTTP communication error occurs,
    /// [`Error::Serialization`] if JSON parsing fails, or
    /// [`Error::Api`] if the Telegram API returned `ok: false`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use telegram_rs_2::core::requests::SendMessage;
    /// use telegram_rs_2::{Bot, Result};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     let bot = Bot::new("TOKEN".to_string());
    ///     let request = SendMessage {
    ///         chat_id: 123456789,
    ///         text: "Hello from telegram-rs!".to_string(),
    ///         parse_mode: None,
    ///         reply_markup: None,
    ///     };
    ///
    ///     let message = bot.execute(request).await?;
    ///     println!("Sent message with ID: {}", message.message_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute<M>(&self, method: M) -> Result<M::Response, Error>
    where
        M: crate::core::requests::Method + serde::Serialize,
    {
        self.client.post(M::NAME, &method).await
    }
}
