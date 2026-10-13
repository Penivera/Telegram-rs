//! Telegram Bot API object types, message models, and keyboard builders.
//!
//! This module defines the Rust representations of Telegram API data structures
//! (such as [`Update`], [`Message`], [`User`], [`Chat`], and [`CallbackQuery`]),
//! along with declarative builders for constructing inline and custom reply keyboards.

use serde::{Deserialize, Serialize};

/// Represents an incoming update from the Telegram Bot API.
///
/// At most one of the optional fields will be present in any given update.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially.
    pub update_id: i32,

    /// New incoming message of any kind — text, photo, sticker, etc.
    pub message: Option<Message>,

    /// New incoming callback query from an inline button press.
    pub callback_query: Option<CallbackQuery>,
}

/// Represents a Telegram chat message.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    /// Unique message identifier inside this chat.
    pub message_id: i32,

    /// For text messages, the actual UTF-8 text of the message.
    pub text: Option<String>,

    /// Conversation the message belongs to.
    pub chat: Chat,

    /// Inline keyboard attached to the message.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// Represents a Telegram chat (private, group, supergroup, or channel).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,

    /// Type of chat, can be `"private"`, `"group"`, `"supergroup"`, or `"channel"`.
    pub r#type: String,
}

/// Represents an incoming callback query from a button on an inline keyboard.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query.
    pub id: String,

    /// Sender of the callback query.
    pub from: User,

    /// Message with the callback button that originated the query, if available.
    pub message: Option<Message>,

    /// Identifier of the message sent via the bot in inline mode, that originated the query.
    pub inline_message_id: Option<String>,

    /// Global identifier, uniquely corresponding to the chat to which the message with the callback button was sent.
    pub chat_instance: String,

    /// Data associated with the callback button. Be aware that the message originated this query can contain no data!
    pub data: Option<String>,

    /// Short name of a Game to be returned, serves as the unique identifier for the game.
    pub game_short_name: Option<String>,
}

/// Represents a Telegram user or bot.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: i64,

    /// `true` if this user is a bot.
    pub is_bot: bool,

    /// User's or bot's first name.
    pub first_name: String,

    /// User's or bot's username (without leading `@`).
    pub username: Option<String>,
}

// ---------------------------------------------------------------------------
// Keyboards & Markup
// ---------------------------------------------------------------------------

/// Represents an inline keyboard that appears directly alongside the message it belongs to.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an array of [`InlineKeyboardButton`] objects.
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// Represents one button of an inline keyboard.
///
/// You must use exactly one of the optional fields.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InlineKeyboardButton {
    /// Label text on the button.
    pub text: String,

    /// HTTP or tg:// URL to be opened when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// Data to be sent in a callback query to the bot when button is pressed (1-64 bytes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,
}

impl InlineKeyboardButton {
    /// Creates a button that emits a callback query when pressed.
    ///
    /// # Arguments
    /// * `text` - Button title.
    /// * `data` - String data payload returned in [`CallbackQuery::data`].
    pub fn callback(text: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: None,
            callback_data: Some(data.into()),
        }
    }

    /// Creates a button that opens a specific URL when pressed.
    ///
    /// # Arguments
    /// * `text` - Button title.
    /// * `url` - Web or deep link URL to open.
    pub fn url(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: Some(url.into()),
            callback_data: None,
        }
    }
}

/// Represents a custom keyboard with reply options.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an array of [`KeyboardButton`] objects.
    pub keyboard: Vec<Vec<KeyboardButton>>,

    /// Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons).
    pub resize_keyboard: bool,

    /// Requests clients to hide the keyboard as soon as it's been used.
    pub one_time_keyboard: bool,
}

/// Represents one button of the reply keyboard.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will be sent as a message when the button is pressed.
    pub text: String,

    /// If `true`, the user's phone number will be sent as a contact when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    /// If `true`, the user's current location will be sent when the button is pressed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
}

impl KeyboardButton {
    /// Creates a standard text button for a reply keyboard.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            request_contact: None,
            request_location: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Keyboard Builders
// ---------------------------------------------------------------------------

/// Fluent builder for constructing [`InlineKeyboardMarkup`] layouts.
///
/// # Examples
///
/// ```
/// use telegram_rs_2::core::types::{InlineKeyboardBuilder, InlineKeyboardButton};
///
/// let keyboard = InlineKeyboardBuilder::new()
///     .row(vec![
///         InlineKeyboardButton::callback("Accept", "action_accept"),
///         InlineKeyboardButton::callback("Decline", "action_decline"),
///     ])
///     .row(vec![
///         InlineKeyboardButton::url("Documentation", "https://docs.rs/telegram-rs-2"),
///     ])
///     .build();
///
/// assert_eq!(keyboard.inline_keyboard.len(), 2);
/// ```
#[derive(Debug, Default)]
pub struct InlineKeyboardBuilder {
    buttons: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardBuilder {
    /// Creates a new, empty `InlineKeyboardBuilder`.
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }

    /// Appends a row of buttons to the inline keyboard layout.
    pub fn row(mut self, buttons: Vec<InlineKeyboardButton>) -> Self {
        self.buttons.push(buttons);
        self
    }

    /// Finalizes and consumes the builder, returning the configured [`InlineKeyboardMarkup`].
    pub fn build(self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup {
            inline_keyboard: self.buttons,
        }
    }
}

/// Fluent builder for constructing [`ReplyKeyboardMarkup`] layouts.
///
/// # Examples
///
/// ```
/// use telegram_rs_2::core::types::{KeyboardButton, ReplyKeyboardBuilder};
///
/// let keyboard = ReplyKeyboardBuilder::new()
///     .row(vec![
///         KeyboardButton::new("Start"),
///         KeyboardButton::new("Help"),
///     ])
///     .resize_keyboard(true)
///     .one_time_keyboard(true)
///     .build();
///
/// assert!(keyboard.resize_keyboard);
/// assert!(keyboard.one_time_keyboard);
/// ```
#[derive(Debug)]
pub struct ReplyKeyboardBuilder {
    buttons: Vec<Vec<KeyboardButton>>,
    resize_keyboard: bool,
    one_time_keyboard: bool,
}

impl Default for ReplyKeyboardBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ReplyKeyboardBuilder {
    /// Creates a new `ReplyKeyboardBuilder` with default sizing and behavior.
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            resize_keyboard: false,
            one_time_keyboard: false,
        }
    }

    /// Appends a row of buttons to the keyboard.
    pub fn row(mut self, buttons: Vec<KeyboardButton>) -> Self {
        self.buttons.push(buttons);
        self
    }

    /// Requests clients to resize the keyboard vertically for optimal fit.
    pub fn resize_keyboard(mut self, resize: bool) -> Self {
        self.resize_keyboard = resize;
        self
    }

    /// Requests clients to hide the keyboard as soon as it has been used.
    pub fn one_time_keyboard(mut self, one_time: bool) -> Self {
        self.one_time_keyboard = one_time;
        self
    }

    /// Finalizes and consumes the builder, returning the configured [`ReplyKeyboardMarkup`].
    pub fn build(self) -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup {
            keyboard: self.buttons,
            resize_keyboard: self.resize_keyboard,
            one_time_keyboard: self.one_time_keyboard,
        }
    }
}
