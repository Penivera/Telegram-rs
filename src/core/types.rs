use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Update {
    pub update_id: i32,
    pub message: Option<Message>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub message_id: i32,
    pub text: Option<String>,
    pub chat: Chat,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chat {
    pub id: i64,
    pub r#type: String, // private, group, etc.
}
