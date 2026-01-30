use crate::Bot;
use crate::core::types::Update;
use crate::Error;

use std::collections::VecDeque;

pub struct Polling {
    bot: Bot,
    timeout: u64,
    offset: Option<i32>,
    buffer: VecDeque<Update>,
}

impl Polling {
    pub fn new(bot: Bot) -> Self {
        Self {
            bot,
            timeout: 10,
            offset: None,
            buffer: VecDeque::new(),
        }
    }

    pub fn timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }

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
