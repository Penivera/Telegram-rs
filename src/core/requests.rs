use serde::{Serialize, de::DeserializeOwned};

pub trait Payload: Serialize {}

pub trait Method: Serialize {
    type Response: DeserializeOwned;
    const NAME: &'static str;
}

#[derive(Serialize)]
pub struct GetUpdates {
    pub offset: Option<i32>,
    pub limit: Option<i32>,
    pub timeout: Option<u64>,
    pub allowed_updates: Option<Vec<String>>,
}

impl Method for GetUpdates {
    type Response = Vec<crate::core::types::Update>;
    const NAME: &'static str = "getUpdates";
}

use crate::core::types::{Message, InlineKeyboardMarkup};

#[derive(Serialize)]
pub struct SendMessage {
    pub chat_id: i64,
    pub text: String,
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>, // Simplified for now, should be Enum
}

impl Method for SendMessage {
    type Response = Message;
    const NAME: &'static str = "sendMessage";
}

#[derive(Serialize)]
pub struct AnswerCallbackQuery {
    pub callback_query_id: String,
    pub text: Option<String>,
    pub show_alert: Option<bool>,
    pub url: Option<String>,
    pub cache_time: Option<i32>,
}

impl Method for AnswerCallbackQuery {
    type Response = bool;
    const NAME: &'static str = "answerCallbackQuery";
}
