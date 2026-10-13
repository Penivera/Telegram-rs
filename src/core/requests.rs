//! Request definitions and traits for Telegram Bot API methods.
//!
//! Every Telegram Bot API method is represented by a struct implementing the [`Method`] trait,
//! declaring its RPC method name and strongly typed expected response.

use crate::core::types::{InlineKeyboardMarkup, Message, Update};
use serde::{de::DeserializeOwned, Serialize};

/// Marker trait for serialized payloads sent to the Telegram API.
pub trait Payload: Serialize {}

/// A Telegram Bot API method request.
///
/// Types implementing `Method` define the endpoint name (e.g. `"sendMessage"`)
/// and the associated response type that Telegram returns upon success.
pub trait Method: Serialize {
    /// The expected response type when the API call succeeds.
    type Response: DeserializeOwned;

    /// The method name as defined in the Telegram Bot API specification (e.g., `"sendMessage"`).
    const NAME: &'static str;
}

/// Request parameters for the [`getUpdates`](https://core.telegram.org/bots/api#getupdates) method.
///
/// Used to retrieve incoming updates using long-polling.
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned.
    /// Must be greater by one than the highest among the identifiers of previously received updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /// Limits the number of updates to be retrieved. Values between 1–100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Timeout in seconds for long polling. Defaults to 0 (short polling).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,

    /// A list of update types you want your bot to receive (e.g., `["message", "callback_query"]`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

impl Method for GetUpdates {
    type Response = Vec<Update>;
    const NAME: &'static str = "getUpdates";
}

/// Request parameters for the [`sendMessage`](https://core.telegram.org/bots/api#sendmessage) method.
///
/// Used to send text messages to a target chat.
///
/// # Examples
///
/// ```
/// use telegram_rs_2::core::requests::SendMessage;
///
/// let message = SendMessage {
///     chat_id: 123456789,
///     text: "Hello, World!".to_string(),
///     parse_mode: Some("MarkdownV2".to_string()),
///     reply_markup: None,
/// };
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct SendMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`).
    pub chat_id: i64,

    /// Text of the message to be sent (1-4096 characters).
    pub text: String,

    /// Mode for parsing entities in the message text (e.g., `"MarkdownV2"`, `"HTML"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// Additional interface options, such as an inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for SendMessage {
    type Response = Message;
    const NAME: &'static str = "sendMessage";
}

/// Request parameters for the [`answerCallbackQuery`](https://core.telegram.org/bots/api#answercallbackquery) method.
///
/// Used to send answers to callback queries sent from inline keyboards.
/// The answer will be displayed to the user as a notification at the top of the chat screen or as an alert.
#[derive(Clone, Debug, Serialize)]
pub struct AnswerCallbackQuery {
    /// Unique identifier for the query to be answered.
    pub callback_query_id: String,

    /// Text of the notification. If not specified, nothing will be shown to the user (0-200 characters).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// If `true`, an alert will be shown by the client instead of a notification at the top of the chat screen. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,

    /// URL that will be opened by the user's client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// The maximum amount of time in seconds that the result of the callback query may be cached client-side. Defaults to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
}

impl Method for AnswerCallbackQuery {
    type Response = bool;
    const NAME: &'static str = "answerCallbackQuery";
}
