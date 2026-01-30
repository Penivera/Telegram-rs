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
