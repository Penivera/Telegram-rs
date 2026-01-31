use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Update {
    pub update_id: i32,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub message_id: i32,
    pub text: Option<String>,
    pub chat: Chat,
    pub reply_markup: Option<InlineKeyboardMarkup>, 
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chat {
    pub id: i64,
    pub r#type: String, // private, group, etc.
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub chat_instance: String,
    pub data: Option<String>,
    pub game_short_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
}

// Keyboards

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub url: Option<String>,
    pub callback_data: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: bool,
    pub one_time_keyboard: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: Option<bool>,
    pub request_location: Option<bool>,
}

// Builders

pub struct InlineKeyboardBuilder {
    buttons: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardBuilder {
    pub fn new() -> Self {
        Self { buttons: Vec::new() }
    }

    pub fn row(mut self, buttons: Vec<InlineKeyboardButton>) -> Self {
        self.buttons.push(buttons);
        self
    }

    pub fn build(self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup { inline_keyboard: self.buttons }
    }
}

impl InlineKeyboardButton {
    pub fn callback(text: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: None,
            callback_data: Some(data.into()),
        }
    }

    pub fn url(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: Some(url.into()),
            callback_data: None,
        }
    }
}

pub struct ReplyKeyboardBuilder {
    buttons: Vec<Vec<KeyboardButton>>,
    resize_keyboard: bool,
    one_time_keyboard: bool,
}

impl ReplyKeyboardBuilder {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            resize_keyboard: false,
            one_time_keyboard: false,
        }
    }

    pub fn row(mut self, buttons: Vec<KeyboardButton>) -> Self {
        self.buttons.push(buttons);
        self
    }

    pub fn resize_keyboard(mut self, resize: bool) -> Self {
        self.resize_keyboard = resize;
        self
    }

    pub fn one_time_keyboard(mut self, one_time: bool) -> Self {
        self.one_time_keyboard = one_time;
        self
    }

    pub fn build(self) -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup {
            keyboard: self.buttons,
            resize_keyboard: self.resize_keyboard,
            one_time_keyboard: self.one_time_keyboard,
        }
    }
}

impl KeyboardButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            request_contact: None,
            request_location: None,
        }
    }

    pub fn request_contact(mut self, request: bool) -> Self {
        self.request_contact = Some(request);
        self
    }

    pub fn request_location(mut self, request: bool) -> Self {
        self.request_location = Some(request);
        self
    }
}
